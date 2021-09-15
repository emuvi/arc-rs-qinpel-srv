use actix_web::error::{Error, ErrorForbidden};
use actix_web::HttpRequest;

use super::SrvData;

pub fn check_access(req: &HttpRequest, srv_data: &SrvData) -> Result<(), Error> {
	if is_origin_local(req) || check_token(req, srv_data) {
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
	host.starts_with("127.0.0.1") || host.starts_with("localhost")
}

pub fn check_token(req: &HttpRequest, srv_data: &SrvData) -> bool {
	let given_token = get_qinpel_token(req);
	if !given_token.is_empty() {
		// TODO - check the token
		return true;
	}
	return false;
}

pub fn get_qinpel_token(req: &HttpRequest) -> String {
	if let Some(token) = req.headers().get("QinpelToken") {
		if let Ok(token) = token.to_str() {
			return String::from(token);
		}
	}
	String::new()
}
