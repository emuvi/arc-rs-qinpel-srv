use actix_web::error::Error;
use actix_web::{web, App, HttpResponse, HttpServer};

use std::sync::{Arc, RwLock};

mod auth;
mod call;
mod clip;
mod data;
mod guard;
mod serve;
mod setup;
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
			.service(serve::ping)
			.service(serve::favicon)
			.service(serve::version)
			.service(auth::login)
			.service(serve::list_app)
			.service(serve::run_app)
			.service(serve::list_cmd)
			.service(serve::run_cmd)
			.service(serve::list_dbs)
			.service(serve::run_dbs)
			.service(serve::shutdown)
	})
	.bind(server_address)?
	.run()
	.await
}
