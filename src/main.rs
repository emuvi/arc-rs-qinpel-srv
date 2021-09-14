use actix_web::{App, HttpServer};

use std::sync::{Arc, RwLock};

mod call;
mod clip;
mod data;
mod guard;
mod serve;
mod setup;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let args = clip::parse();
	if args.is_present("no-run") {
		std::process::exit(0);
	}
	println!("QinpelSrv running...");
	let setup = setup::Head::load(args);
	let server_address = format!("{}:{}", setup.host, setup.port);
	println!("Server address: {}", server_address);
	let data = Arc::new(RwLock::new(data::Body::new(setup)));
	HttpServer::new(move || {
		App::new()
			.data(data.clone())
			.service(serve::ping)
			.service(serve::version)
			.service(serve::shutdown)
			.service(serve::favicon)
			.service(serve::list_apps)
			.service(serve::run_apps())
			.service(serve::list_cmds)
			.service(serve::run_cmds)
	})
	.bind(server_address)?
	.run()
	.await
}


