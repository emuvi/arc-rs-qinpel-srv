use actix_web::error::{Error, ErrorForbidden};
use actix_web::HttpRequest;

use std::path::PathBuf;

use super::data::Access;
use super::data::User;
use super::SrvData;

pub fn get_user(req: &HttpRequest, srv_data: &SrvData) -> Option<User> {
	if is_origin_local(req) {
		let users = &srv_data.read().unwrap().users;
		if let Some(root) = users.into_iter().find(|user| user.name == "root") {
			return Some(root.clone());
		}
	}
	get_token_user(req, srv_data)
}

pub fn check_run_access(req: &HttpRequest, srv_data: &SrvData) -> Result<(), Error> {
	if is_origin_local(req) || check_run_token(req, srv_data) {
		return Ok(());
	} else {
		return Err(ErrorForbidden(
			"You don't have access to call this resource.",
		));
	}
}

pub fn check_dir_access(
	path_ref: &PathBuf,
	path_dest: Option<&PathBuf>,
	resource: &str,
	user: &User,
) -> Result<(), Error> {
	let path_ref = format!("{}", path_ref.display());
	let path_dest: Option<String> = if path_dest.is_some() {
		Some(format!("{}", path_dest.unwrap().display()))
	} else {
		None
	};
	if check_dir_resource(path_ref, path_dest, resource, user) {
		return Ok(());
	} else {
		return Err(ErrorForbidden(
			"You don't have access to call this resource.",
		));
	}
}

pub fn is_origin_local(req: &HttpRequest) -> bool {
	let info = req.connection_info();
	let host = info.host();
	host.starts_with("localhost") || host.starts_with("127.0.0.1")
}

pub fn check_run_token(req: &HttpRequest, srv_data: &SrvData) -> bool {
	let req_path = req.match_info().path();
	if req_path.starts_with("/run/app/qinpel-app/") {
		return true;
	}
	let token_user = get_token_user(req, srv_data);
	if token_user.is_none() {
		return false;
	}
	let user = token_user.unwrap();
	if user.master {
		return true;
	}
	for user_access in &user.access {
		match user_access {
			Access::APP { name } => {
				if req_path.starts_with(&format!("/run/app/{}/", name)) {
					return true;
				}
			}
			Access::CMD { name, params: _ } => {
				if req_path.starts_with(&format!("/run/cmd/{}/", name)) {
					return true;
				}
			}
			Access::DBS { name } => {
				if req_path.starts_with(&format!("/run/dbs/{}/", name)) {
					return true;
				}
			}
			_ => {}
		}
	}
	return false;
}

pub fn check_dir_resource(
	path_ref: String,
	path_dest: Option<String>,
	resource: &str,
	user: &User,
) -> bool {
	if user.master {
		return true;
	}
	if resource == "/dir/list" {
		return check_dir_read(&user, &path_ref);
	} else if resource == "/dir/new" {
		return check_dir_write(&user, &path_ref);
	} else if resource == "/dir/copy" {
		if let Some(path_dest) = path_dest {
			return check_dir_read(&user, &path_ref) && check_dir_write(&user, &path_dest);
		}
	} else if resource == "/dir/move" {
		if let Some(path_dest) = path_dest {
			return check_dir_write(&user, &path_ref) && check_dir_write(&user, &path_dest);
		}
	} else if resource == "/dir/del" {
		return check_dir_write(&user, &path_ref);
	} else if resource == "/file/read" {
		return check_dir_read(&user, &path_ref);
	} else if resource == "/file/write" {
		return check_dir_write(&user, &path_ref);
	} else if resource == "/file/append" {
		return check_dir_write(&user, &path_ref);
	} else if resource == "/file/copy" {
		if let Some(path_dest) = path_dest {
			return check_dir_read(&user, &path_ref) && check_dir_write(&user, &path_dest);
		}
	} else if resource == "/file/move" {
		if let Some(path_dest) = path_dest {
			return check_dir_write(&user, &path_ref) && check_dir_write(&user, &path_dest);
		}
	} else if resource == "/file/del" {
		return check_dir_write(&user, &path_ref);
	}
	false
}

pub fn check_dir_read(user: &User, check_path: &str) -> bool {
	for user_access in &user.access {
		if let Access::DIR { path, write: _ } = user_access {
			if check_path.starts_with(path) {
				return true;
			}
		}
	}
	false
}

pub fn check_dir_write(user: &User, check_path: &str) -> bool {
	for user_access in &user.access {
		if let Access::DIR { path, write } = user_access {
			if check_path.starts_with(path) && *write {
				return true;
			}
		}
	}
	false
}

pub fn get_token_user(req: &HttpRequest, srv_data: &SrvData) -> Option<User> {
	let got_token = get_qinpel_token(req);
	if got_token.is_empty() {
		return None;
	}
	let our_tokens = &srv_data.read().unwrap().tokens;
	let found_auth = our_tokens.get(&got_token);
	if found_auth.is_none() {
		return None;
	}
	let found_auth = found_auth.unwrap();
	return Some(found_auth.user.clone());
}

pub fn get_qinpel_token(req: &HttpRequest) -> String {
	if let Some(token) = req.headers().get("Qinpel-Token") {
		if let Ok(token) = token.to_str() {
			return String::from(token);
		}
	}
	String::new()
}
