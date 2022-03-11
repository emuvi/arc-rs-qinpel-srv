pub use actix_web;
pub use liz;

use actix_web::dev::Service;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::future::FutureExt;
use liz::liz_dbg_errs;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod data;
mod dirs;
mod files;
mod guard;
mod lists;
mod login;
mod persist;
mod pooling;
mod precept;
mod runner;
mod server;
mod servfs;
mod setup;

type SrvData = web::Data<Arc<data::Body>>;
type SrvError = actix_web::error::Error;
type SrvResult = Result<HttpResponse, SrvError>;

pub static DEBUG: AtomicBool = AtomicBool::new(false);
pub static VERBOSE: AtomicBool = AtomicBool::new(false);

pub struct QinServer {
    pub debug: Option<bool>,
    pub verbose: Option<bool>,
    pub server_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u64>,
    pub serves_pubs: Option<bool>,
    pub serves_apps: Option<bool>,
    pub serves_dirs: Option<bool>,
    pub serves_cmds: Option<bool>,
    pub serves_sqls: Option<bool>,
    pub serves_lizs: Option<bool>,
    pub redirects: Option<HashMap<String, String>>,
}

pub async fn start(qin_server: QinServer) -> std::io::Result<()> {
    let setup = setup::Head::load(qin_server);
    let server_address = format!("{}:{}", setup.server_host, setup.server_port);
    let body = data::Body::new(setup);
    if body.head.verbose {
        println!("{} starting...", body.head.server_name);
        println!("Server head: {:?}", body.head);
        println!("Server has {} user(s).", body.users.len());
        println!("{:?}", body.users);
        println!("Server has {} base(s).", body.bases.len());
        println!("{:?}", body.bases);
    }
    let data = Arc::new(body);
    let data_main = data.clone();
    let server = HttpServer::new(move || {
        let server_app = App::new()
            .wrap_fn(|req, srv| {
                let should_log = DEBUG.load(Ordering::Relaxed);
                let log_req: Option<String> = if should_log {
                    Some(format!("Request: \n{:?}", req))
                } else {
                    None
                };
                srv.call(req).map(|res| {
                    if let Some(log_req) = log_req {
                        println!("{}\nResponse: \n{:?}", log_req, res);
                    }
                    res
                })
            })
            .wrap(if VERBOSE.load(Ordering::Relaxed) {
                middleware::DefaultHeaders::new()
                    .header("version", env!("CARGO_PKG_VERSION"))
            } else {
                middleware::DefaultHeaders::new()
            })
            .data(data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest().body(liz_dbg_errs!(err)),
                )
                .into()
            }))
            .service(server::ping)
            .service(server::stop)
            .service(server::shut)
            .service(server::version)
            .service(login::enter);
        let server_app = if data.head.serves_pubs {
                server_app.service(runner::get_pub)
            } else {
                server_app
            };
        let server_app = if data.head.serves_apps {
            server_app.service(runner::get_app).service(runner::list_apps)
        } else {
            server_app
        };
        let server_app = if data.head.serves_dirs {
            server_app.service(servfs::dir_list)
                .service(servfs::dir_new)
                .service(servfs::dir_copy)
                .service(servfs::dir_move)
                .service(servfs::dir_del)
                .service(servfs::file_read)
                .service(servfs::file_write)
                .service(servfs::file_append)
                .service(servfs::file_copy)
                .service(servfs::file_move)
                .service(servfs::file_del)
        } else {
            server_app
        };
        let server_app = if data.head.serves_cmds {
            server_app.service(runner::run_cmd).service(runner::list_cmds)
        } else {
            server_app
        };
        let server_app = if data.head.serves_sqls {
            server_app.service(runner::run_sql)
                .service(runner::ask_sql)
                .service(runner::list_bases)
        } else {
            server_app
        };
        server_app.service(server::redirect)
    });
    let secure = secure_server();
    let runner = if let Some(config) = secure {
        server.bind_rustls(server_address, config)?.run()
    } else {
        server.bind(server_address)?.run()
    };
    let data_runner = runner.clone();
    {
        let mut data_server = data_main.server.write().unwrap();
        *data_server = Some(data_runner);
    }
    runner.await
}

fn secure_server() -> Option<ServerConfig> {
    let cert_path = Path::new("key/cert.pem");
    let key_path = Path::new("key/key.pem");
    if cert_path.exists() && key_path.exists() {
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open(cert_path).unwrap());
        let key_file = &mut BufReader::new(File::open(key_path).unwrap());
        let cert_chain = certs(cert_file).unwrap();
        let mut keys = pkcs8_private_keys(key_file).unwrap();
        if keys.is_empty() {
            eprintln!("Could not locate PKCS 8 private keys.");
            std::process::exit(1);
        }
        config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
        return Some(config);
    }
    None
}
