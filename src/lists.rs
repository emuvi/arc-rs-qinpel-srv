use actix_web::{HttpRequest, HttpResponse};

use std::path::Path;

use crate::bad_srv;
use crate::data::Access;
use crate::guard;
use crate::SrvData;
use crate::SrvError;
use crate::SrvResult;

pub fn list_apps(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	let user = guard::get_user_or_err(req, srv_data)?;
	let apps_dir = liz::liz_paths::path_join(&srv_data.srv_dir, "app").map_err(|err| bad_srv(err))?;
	let apps_dirs = list_folder_dirs(Path::new(&apps_dir))?;
	let mut body = String::new();
	if user.master {
		apps_dirs.into_iter().for_each(|dir| {
			body.push_str(&dir);
			body.push_str("\n");
		});
	} else {
		for user_access in &user.access {
			match user_access {
				Access::APP { name } => {
					if apps_dirs.contains(name) {
						body.push_str(name);
						body.push_str("\n");
					}
				}
				_ => {}
			}
		}
	}
	Ok(HttpResponse::Ok().body(body))
}

pub fn list_cmds(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	let user = guard::get_user_or_err(req, srv_data)?;
	let cmds_dir = liz::liz_paths::path_join(&srv_data.srv_dir, "app").map_err(|err| bad_srv(err))?;
	let cmds_dirs = list_folder_dirs(Path::new(&cmds_dir))?;
	let mut body = String::new();
	if user.master {
		cmds_dirs.into_iter().for_each(|dir| {
			body.push_str(&dir);
			body.push_str("\n");
		});
	} else {
		for user_access in &user.access {
			match user_access {
				Access::CMD { name, args: _ } => {
					if cmds_dirs.contains(name) {
						body.push_str(name);
						body.push_str("\n");
					}
				}
				_ => {}
			}
		}
	}
	Ok(HttpResponse::Ok().body(body))
}

fn list_folder_dirs(folder: &Path) -> Result<Vec<String>, SrvError> {
	let mut result = Vec::new();
	if folder.exists() {
		for entry in folder.read_dir()? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				if let Some(name) = path.file_name() {
					if let Some(name) = name.to_str() {
						result.push(String::from(name));
					}
				}
			}
		}
	}
	Ok(result)
}

pub fn list_bases(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	let user = guard::get_user_or_err(req, srv_data)?;
	if user.master {
		return list_all_bases(srv_data);
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
	Ok(HttpResponse::Ok().body(body))
}

fn list_all_bases(srv_data: &SrvData) -> SrvResult {
	let mut body = String::new();
	let bases = &srv_data.bases;
	for base in bases {
		body.push_str(&base.name);
		body.push_str("\n");
	}
	Ok(HttpResponse::Ok().body(body))
}
