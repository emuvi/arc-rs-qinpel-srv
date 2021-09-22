use actix_web::error::ErrorForbidden;
use actix_web::HttpRequest;

use std::path::PathBuf;

use super::SrvData;
use super::SrvResult;

pub fn read(path: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn write(path: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn append(path: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn copy(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn mov(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}

pub fn del(path: PathBuf) -> SrvResult {
	Err(ErrorForbidden("We don't to check access here."))
}
