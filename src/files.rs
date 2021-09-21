use actix_web::error::ErrorForbidden;
use actix_web::HttpRequest;

use super::SrvData;
use super::SrvResult;

pub fn read(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn write(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn append(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn copy(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn mov(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn del(path: String, req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}
