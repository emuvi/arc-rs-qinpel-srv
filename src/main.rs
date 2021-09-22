use actix_web::error::Error;
use actix_web::{web, App, HttpResponse, HttpServer};

use std::sync::{Arc, RwLock};

mod auth;
mod clip;
mod data;
mod dirs;
mod files;
mod guard;
mod lists;
mod maker;
mod server;
mod servfs;
mod setup;
mod texts;
mod utils;

type SrvData = web::Data<Arc<RwLock<data::Body>>>;
type SrvResult = Result<HttpResponse, Error>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let args = clip::parse();
	if args.is_present("no-run") {
		std::process::exit(0);
	}
	println!("QinpelSrv starting...");
	let setup = setup::Head::load(args);
	let server_address = format!("{}:{}", setup.host, setup.port);
	println!("Server address: {}", server_address);
	let data = Arc::new(RwLock::new(data::Body::new(setup)));
	HttpServer::new(move || {
		App::new()
			.data(data.clone())
			.service(auth::login)
			.service(server::ping)
			.service(server::favicon)
			.service(server::version)
			.service(server::shutdown)
			.service(server::list_app)
			.service(server::run_app)
			.service(server::list_cmd)
			.service(server::run_cmd)
			.service(server::list_dbs)
			.service(server::run_dbs)
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
			.service(texts::translate)
	})
	.bind(server_address)?
	.run()
	.await
}
