use actix_files as actix_fs;
use actix_web::error::{Error, ErrorForbidden};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::Path;

mod clip;
mod setup;

#[get("/")]
async fn index() -> impl Responder {
	let mut body = String::from("QinpelSrv is running on version: ");
	body.push_str(clap::crate_version!());
	HttpResponse::Ok().body(body)
}

fn read_token() -> String {
	let path = Path::new("token.txt");
	if path.exists() {
		fs::read_to_string("token.txt").unwrap()
	} else {
		let new_token: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(27)
			.map(char::from)
			.collect();
		fs::write(path, &new_token).unwrap();
		new_token
	}
}

#[get("/reboot")]
async fn reboot(req: HttpRequest) -> Result<impl Responder, Error> {
	if let Some(origin) = req.headers().get("ORIGIN") {
		println!("{}", origin.to_str().unwrap());	
	}
	if let Some(token) = req.headers().get("token") {
		let our_token = read_token();
		let given_token = token.to_str().unwrap();
		if our_token == given_token {
			reboot_server();
			return Ok("QinpelSrv is rebooting...".to_owned());
		} else {
			return Err(ErrorForbidden("You must inform the correct token."));
		}
	} else {
		return Err(ErrorForbidden("You must inform the token."));
	}
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
			.service(reboot)
			.service(actix_fs::Files::new("/run/apps", "./run/apps").index_file("index.html"))
	})
	// TODO - This causes undefined behavior on windows if is called on the same address more than one time.
	.bind(format!("{}:{}", setup.host, setup.port))?
	.run()
	.await
}

fn reboot_server() {
	println!("Rebooting the server...");
}
