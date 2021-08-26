use actix_files as actix_fs;
use actix_web::error::{Error, ErrorForbidden};
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[get("/")]
async fn index() -> impl Responder {
	HttpResponse::Ok().body("QinpelSrv is running on version: v0.1.0")
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
	println!("QinpelSrv starting...");
	let path = Path::new("setup.json");
	let mut setup = Value::Null;
	if path.exists() {
		let file = std::fs::File::open(path).unwrap();
		setup = serde_json::from_reader(file).unwrap();
	}
	let host = if setup == Value::Null {
		String::from("0.0.0.0")
	} else {
		format!("{}", setup["serverHost"].as_str().unwrap())
	};
	let port = if setup == Value::Null {
		String::from("5490")
	} else {
		format!("{}", setup["serverPort"])
	};
	println!("Server host: {}", host);
	println!("Server port: {}", port);
	HttpServer::new(|| {
		App::new()
			.service(index)
			.service(reboot)
			.service(actix_fs::Files::new("/apps", "./run/apps").index_file("index.html"))
	})
	.bind(format!("{}:{}", host, port))?
	.run()
	.await
}

fn reboot_server() {
	println!("Rebooting the server...");
}
