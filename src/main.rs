use actix_files as actix_fs;
use actix_web::error::{Error, ErrorForbidden};
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::Path;
use std::time::Duration;

mod clip;
mod setup;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(1000);

fn is_origin_local(req: &HttpRequest) -> bool {
	let info = req.connection_info();
	let host = info.host();
	host.starts_with("127.0.0.1") || host.starts_with("localhost")
}

fn read_master_token() -> String {
	let path = Path::new("token.txt");
	if path.exists() {
		return fs::read_to_string("token.txt").expect("Error: Could not read the token file.");
	} else {
		let new_token: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(18)
			.map(char::from)
			.collect();
		fs::write(path, &new_token).expect("Error: Could not write the token file.");
		return new_token;
	}
}

fn check_master_token(req: &HttpRequest) -> bool {
	if let Some(token) = req.headers().get("token") {
		let our_token = read_master_token();
		let given_token = token.to_str().unwrap();
		if our_token == given_token {
			return true;
		}
	}
	return false;
}

fn check_access(req: &HttpRequest) -> bool {
	is_origin_local(req) || check_master_token(req)
}

#[get("/")]
async fn index(req: HttpRequest) -> Result<impl Responder, Error> {
	if !check_access(&req) {
		return Err(ErrorForbidden(
			"You don't have access to call this resource.",
		));
	}
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
async fn shutdown(req: HttpRequest) -> Result<impl Responder, Error> {
	if !check_access(&req) {
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
	println!("Server host: {}", setup.host);
	println!("Server port: {}", setup.port);
	HttpServer::new(|| {
		App::new()
			.service(index)
			.service(ping)
			.service(version)
			.service(shutdown)
			.service(actix_fs::Files::new("/run/apps", "./run/apps").index_file("index.html"))
	})
	.bind(format!("{}:{}", setup.host, setup.port))?
	.run()
	.await
}

fn call_index() -> Result<impl Responder, Error> {
	let mut body = String::from("QinpelSrv is serving:\n");
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
