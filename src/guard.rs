use actix_web::error::{Error, ErrorForbidden};
use actix_web::HttpRequest;

use super::data::User;
use super::SrvData;

pub fn check_run_access(req: &HttpRequest, srv_data: &SrvData) -> Result<(), Error> {
	if is_origin_local(req) || check_run_token(req, srv_data) {
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
	let token_user = token_user.unwrap();
	if token_user.master {
		return true;
	}
	for user_access in &token_user.access {
		if req_path.starts_with(&format!("{}", user_access)) {
			return true;
		}
	}
	return false;
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
	if let Some(token) = req.headers().get("QinpelToken") {
		if let Ok(token) = token.to_str() {
			return String::from(token);
		}
	}
	String::new()
}
