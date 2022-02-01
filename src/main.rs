use actix_web::{web, App, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

mod clip;
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
mod trans;
mod utils;

type SrvData = web::Data<Arc<data::Body>>;
type SrvError = actix_web::error::Error;
type SrvResult = Result<HttpResponse, SrvError>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let args = clip::parse();
	if args.is_present("no-run") {
		std::process::exit(0);
	}
	println!("QinpelSrv loading...");
	let setup = setup::Head::load(args);
	println!("Server setup: {:?}", setup);
	let server_address = format!("{}:{}", setup.host, setup.port);
	let body = data::Body::new(setup);
	println!("Server has {} user(s).", body.users.len());
	println!("Server has {} base(s).", body.bases.len());
	let data = Arc::new(body);
	let data_main = data.clone();
	println!("QinpelSrv starting...");
	let server = HttpServer::new(move || {
		App::new()
			.data(data.clone())
			.app_data(web::JsonConfig::default().error_handler(|err, _req| {
				actix_web::error::InternalError::from_response(
					"",
					HttpResponse::BadRequest().body(utils::debug(utils::origin!(), &err)),
				)
				.into()
			}))
			.service(server::ping)
			.service(server::favicon)
			.service(server::version)
			.service(server::stop)
			.service(server::shut)
			.service(login::enter)
			.service(runner::list_app)
			.service(runner::run_app)
			.service(runner::list_cmd)
			.service(runner::run_cmd)
			.service(runner::list_dbs)
			.service(runner::run_dbs)
			.service(runner::ask_dbs)
			.service(servfs::dir_list)
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
			.service(trans::translate)
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
		println!("QinpelSrv securing...");
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
