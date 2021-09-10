use actix_files as actix_fs;
use actix_web::error::{Error, ErrorForbidden};
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

mod clip;
mod data;
mod guard;
mod setup;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(1000);

#[get("/index")]
async fn index() -> Result<impl Responder, Error> {
	call_index()
}

#[get("/ping")]
async fn ping() -> impl Responder {
	"QinpelSrv pong..."
}

#[get("/version")]
async fn version() -> impl Responder {
	let mut body = String::from("QinpelSrv is running on version: ");
	body.push_str(clap::crate_version!());
	body
}

#[get("/shutdown")]
async fn shutdown(
	req: HttpRequest,
	srv_data: web::Data<Arc<RwLock<data::Body>>>,
) -> Result<impl Responder, Error> {
	if !guard::check_access(&req, &srv_data) {
		return Err(ErrorForbidden(
			"You don't have access to call this resource.",
		));
	}
	call_shutdown()
}

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
			.service(index)
			.service(ping)
			.service(version)
			.service(shutdown)
			.service(actix_fs::Files::new("/run/apps", "./run/apps"))
	})
	.bind(server_address)?
	.run()
	.await
}

fn call_index() -> Result<impl Responder, Error> {
	let mut body = String::from("QinpelSrv apps:\n");
	for entry in Path::new("./run/apps").read_dir()? {
		if let Ok(entry) = entry {
			let path = entry.path();
			if path.is_dir() {
				if let Some(name) = path.file_name() {
					if let Some(name) = name.to_str() {
						body.push_str(name);
						body.push_str("\n");
					}
				}
			}
		}
	}
	Ok(body)
}

fn call_shutdown() -> Result<impl Responder, Error> {
	let result = String::from("QinpelSrv is shutdown...");
	println!("{}", result);
	std::thread::spawn(|| {
		std::thread::sleep(SLEEP_TO_SHUTDOWN);
		std::process::exit(0);
	});
	Ok(result)
}
