use actix_web::{HttpRequest, HttpResponse};

use std::path::{Path, PathBuf};
use std::time::Duration;

use super::data::Access;
use super::guard;
use super::SrvData;
use super::SrvResult;

fn list_folder(folder: PathBuf) -> SrvResult {
	let mut body = String::new();
	for entry in folder.read_dir()? {
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
	Ok(HttpResponse::Ok().body(body))
}

pub fn list_app(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if guard::is_origin_local(req) {
		return list_folder(Path::new("./run/app").to_owned());
	}
	if let Some(token_user) = guard::get_token_user(req, srv_data) {
		if token_user.master {
			return list_folder(Path::new("./run/app").to_owned());
		}
		let mut body = String::new();
		for user_access in &token_user.access {
			match user_access {
				Access::APP { name } => {
					body.push_str(name);
					body.push_str("\n");
				}
				_ => {}
			}
		}
		return Ok(HttpResponse::Ok().body(body));
	}
	Ok(HttpResponse::Ok().body(String::new()))
}

pub fn list_cmd(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if guard::is_origin_local(req) {
		return list_folder(Path::new("./run/cmd").to_owned());
	}
	if let Some(token_user) = guard::get_token_user(req, srv_data) {
		if token_user.master {
			return list_folder(Path::new("./run/cmd").to_owned());
		}
		let mut body = String::new();
		for user_access in &token_user.access {
			match user_access {
				Access::CMD { name, params: _ } => {
					body.push_str(name);
					body.push_str("\n");
				}
				_ => {}
			}
		}
		return Ok(HttpResponse::Ok().body(body));
	}
	Ok(HttpResponse::Ok().body(String::new()))
}

fn list_all_dbs(srv_data: &SrvData) -> SrvResult {
	let mut body = String::new();
	let bases = &srv_data.read().unwrap().bases;
	for base in bases {
		body.push_str(&base.name);
		body.push_str("\n");
	}
	Ok(HttpResponse::Ok().body(body))
}

pub fn list_dbs(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if guard::is_origin_local(req) {
		return list_all_dbs(srv_data);
	}
	if let Some(token_user) = guard::get_token_user(req, srv_data) {
		if token_user.master {
			return list_all_dbs(srv_data);
		}
		let mut body = String::new();
		for user_access in &token_user.access {
			match user_access {
				Access::DBS { name } => {
					body.push_str(name);
					body.push_str("\n");
				}
				_ => {}
			}
		}
		return Ok(HttpResponse::Ok().body(body));
	}
	Ok(HttpResponse::Ok().body(String::new()))
}

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(3000);

pub fn shutdown() -> SrvResult {
	let result = String::from("QinpelSrv is shutdown...");
	println!("{}", result);
	std::thread::spawn(|| {
		std::thread::sleep(SLEEP_TO_SHUTDOWN);
		std::process::exit(0);
	});
	Ok(HttpResponse::Ok().body(result))
}
