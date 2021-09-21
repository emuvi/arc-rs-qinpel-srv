use actix_web::error::ErrorForbidden;
use actix_web::{HttpRequest, HttpResponse};

use std::path::{Path, PathBuf};

use super::data::Access;
use super::guard;
use super::SrvData;
use super::SrvResult;

pub fn list_app(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if let Some(user) = guard::get_user(req, srv_data) {
		if user.master {
			return list_folder_dirs(Path::new("./run/app").to_owned());
		}
		let mut body = String::new();
		for user_access in &user.access {
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
	Err(ErrorForbidden(
		"You don't have access to call this resource.",
	))
}

pub fn list_cmd(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if let Some(user) = guard::get_user(req, srv_data) {
		if user.master {
			return list_folder_dirs(Path::new("./run/cmd").to_owned());
		}
		let mut body = String::new();
		for user_access in &user.access {
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
	Err(ErrorForbidden(
		"You don't have access to call this resource.",
	))
}

fn list_folder_dirs(folder: PathBuf) -> SrvResult {
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

pub fn list_dbs(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if let Some(user) = guard::get_user(req, srv_data) {
		if user.master {
			return list_all_dbs(srv_data);
		}
		let mut body = String::new();
		for user_access in &user.access {
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
	Err(ErrorForbidden(
		"You don't have access to call this resource.",
	))
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
