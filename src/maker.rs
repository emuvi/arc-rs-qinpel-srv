use actix_web::error::ErrorForbidden;
use actix_web::{HttpRequest, HttpResponse};

use std::time::Duration;

use super::guard;
use super::SrvData;
use super::SrvResult;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(3000);

pub fn shutdown(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if let Some(user) = guard::get_user(req, srv_data) {
		if user.master {
			let result = String::from("QinpelSrv is shutdown...");
			println!("{}", result);
			std::thread::spawn(|| {
				std::thread::sleep(SLEEP_TO_SHUTDOWN);
				std::process::exit(0);
			});
			return Ok(HttpResponse::Ok().body(result));
		}
	}
	Err(ErrorForbidden("You don't have access to call this resource."))
}

pub fn run_cmd(body: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("You don't have access to call this resource."))
}

pub fn run_dbs(body: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("You don't have access to call this resource."))
}