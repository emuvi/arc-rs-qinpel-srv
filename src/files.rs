use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{HttpResponse};

use std::fs;
use std::path::PathBuf;

use super::SrvResult;

pub fn read(path: PathBuf) -> Result<NamedFile, Error> {
	Ok(NamedFile::open(path)?)
}

pub fn write(path: PathBuf, base64: bool, data: &String) -> SrvResult {
	Ok(HttpResponse::Ok().body("We don't to check access here."))
}

pub fn append(path: PathBuf, base64: bool, data: &String) -> SrvResult {
	Ok(HttpResponse::Ok().body("We don't to check access here."))
}

pub fn copy(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	if !origin.exists() {
		return Err(ErrorBadRequest(
			"The file origin to copy does not exists.",
		));
	}
	if origin.is_dir() {
		return Err(ErrorBadRequest(
			"The file origin to copy is a directory.",
		));
	}
	fs::copy(&origin, &destiny)?;
	Ok(HttpResponse::Ok().body(format!(
		"File copied from: {} to: {}",
		origin.display(),
		destiny.display()
	)))
}

pub fn mov(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	if !origin.exists() {
		return Err(ErrorBadRequest(
			"The file origin to copy does not exists.",
		));
	}
	if origin.is_dir() {
		return Err(ErrorBadRequest(
			"The file origin to copy is a directory.",
		));
	}
	fs::copy(&origin, &destiny)?;
	fs::remove_file(&origin)?;
	Ok(HttpResponse::Ok().body(format!(
		"File moved from: {} to: {}",
		origin.display(),
		destiny.display()
	)))
}

pub fn del(path: PathBuf) -> SrvResult {
	if !path.exists() {
		return Err(ErrorBadRequest("The file to delete does not exists."));
	}
	if path.is_dir() {
		return Err(ErrorBadRequest("The file to delete is a directory."));
	}
	fs::remove_file(&path)?;
	Ok(HttpResponse::Ok().body(format!("File deleted: {}", path.display())))
}
