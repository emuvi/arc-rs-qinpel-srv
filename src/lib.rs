pub use actix_web;
pub use liz;

use actix_web::dev::Service;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, App, HttpResponse, HttpServer};
use futures::future::FutureExt;
use liz::liz_dbg_errs;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

mod auth;
mod base;
mod body;
mod conf;
mod dirs;
mod files;
mod guard;
mod lists;
mod persist;
mod pooling;
mod precept;
mod srvauth;
mod srvbase;
mod srvdirs;
mod srvruns;
mod srvutil;

type SrvData = web::Data<Arc<body::Body>>;
type SrvError = actix_web::error::Error;
type SrvResult = Result<HttpResponse, SrvError>;

pub struct QinServer {
    pub verbose: Option<bool>,
    pub archive: Option<bool>,
    pub server_name: Option<String>,
    pub server_host: Option<String>,
    pub server_port: Option<u64>,
    pub serves_pubs: Option<bool>,
    pub serves_apps: Option<bool>,
    pub serves_dirs: Option<bool>,
    pub serves_cmds: Option<bool>,
    pub serves_regs: Option<bool>,
    pub serves_sqls: Option<bool>,
    pub serves_lizs: Option<bool>,
    pub redirects: Option<HashMap<String, String>>,
}

pub async fn start(qin_server: QinServer) -> std::io::Result<()> {
    let setup = conf::Head::load(qin_server);
    let server_address = format!("{}:{}", setup.server_host, setup.server_port);
    let body = body::Body::new(setup);
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
        let server_app = App::new();
        #[cfg(debug_assertions)]
        let server_app = server_app.wrap_fn(|req, srv| {
            let start = format!("\nRequest: \n{:?}", req);
            srv.call(req).map(move |res| {
                let finish = format!("{}\nResponse: \n{:?}", start, res);
                liz::liz_debug::debug(finish);
                res
            })
        });
        let server_app = server_app
            .data(data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest().body(liz_dbg_errs!(err)),
                )
                .into()
            }))
            .service(srvauth::enter)
            .service(srvauth::exit);
        let server_app = if data.head.serves_pubs {
            server_app.service(srvruns::pub_get)
        } else {
            server_app
        };
        let server_app = if data.head.serves_apps {
            server_app
                .service(srvruns::app_get)
                .service(srvruns::list_apps)
        } else {
            server_app
        };
        let server_app = if data.head.serves_dirs {
            server_app
                .service(srvdirs::dir_list)
                .service(srvdirs::dir_new)
                .service(srvdirs::dir_copy)
                .service(srvdirs::dir_move)
                .service(srvdirs::dir_del)
                .service(srvdirs::file_read)
                .service(srvdirs::file_write)
                .service(srvdirs::file_append)
                .service(srvdirs::file_copy)
                .service(srvdirs::file_move)
                .service(srvdirs::file_del)
        } else {
            server_app
        };
        let server_app = if data.head.serves_cmds {
            server_app
                .service(srvruns::cmd_run)
                .service(srvruns::list_cmds)
        } else {
            server_app
        };
        let server_app = if data.head.serves_base() {
            server_app.service(srvbase::list_bases)
        } else {
            server_app
        };
        let server_app = if data.head.serves_regs {
            server_app
                .service(srvbase::reg_new)
                .service(srvbase::reg_ask)
                .service(srvbase::reg_set)
                .service(srvbase::reg_del)
        } else {
            server_app
        };
        let server_app = if data.head.serves_sqls {
            server_app
                .service(srvbase::sql_run)
                .service(srvbase::sql_ask)
        } else {
            server_app
        };
        let server_app = if data.head.serves_lizs {
            server_app.service(srvruns::liz_run)
        } else {
            server_app
        };
        server_app
            .service(srvutil::ping)
            .service(srvutil::version)
            .service(srvutil::stop)
            .service(srvutil::shut)
            .service(srvutil::redirect)
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

pub fn bad_req(err: impl std::fmt::Display) -> actix_web::error::Error {
    ErrorBadRequest(format!("{}", err))
}

pub fn bad_srv(err: impl std::fmt::Display) -> actix_web::error::Error {
    ErrorInternalServerError(format!("{}", err))
}
