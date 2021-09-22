use actix_web::error::{ErrorBadRequest};
use actix_web::{HttpResponse};

use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use super::SrvResult;

pub fn list(path: PathBuf) -> SrvResult {
	if !path.exists() {
		return Err(ErrorBadRequest("The folder to list does not exists."));
	}
	if !path.is_dir() {
		return Err(ErrorBadRequest("The folder to list is not a directory."));
	}
	let mut body = String::new();
	for entry in path.read_dir()? {
		let entry = entry?;
		let inside = entry.path();
		if let Some(name) = inside.file_name() {
			if let Some(name) = name.to_str() {
				body.push_str(if inside.is_dir() { "D: " } else { "F: " });
				body.push_str(name);
				body.push_str("\n");
			}
		}
	}
	Ok(HttpResponse::Ok().body(body))
}

pub fn new(path: PathBuf) -> SrvResult {
	fs::create_dir_all(&path)?;
	Ok(HttpResponse::Ok().body(format!("Folder created: {}", path.display())))
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
	fs::create_dir_all(&dst)?;
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let file_type = entry.file_type()?;
		if file_type.is_dir() {
			copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
		} else {
			fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
		}
	}
	Ok(())
}

pub fn copy(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	let origin = Path::new(&origin);
	if !origin.exists() {
		return Err(ErrorBadRequest(
			"The folder origin to copy does not exists.",
		));
	}
	if !origin.is_dir() {
		return Err(ErrorBadRequest(
			"The folder origin to copy is not a directory.",
		));
	}
	let destiny = Path::new(&destiny);
	copy_dir_all(origin, destiny)?;
	Ok(HttpResponse::Ok().body(format!(
		"Folder copied from: {} to: {}",
		origin.display(),
		destiny.display()
	)))
}

pub fn mov(origin: PathBuf, destiny: PathBuf) -> SrvResult {
	let origin = Path::new(&origin);
	if !origin.exists() {
		return Err(ErrorBadRequest(
			"The folder origin to copy does not exists.",
		));
	}
	if !origin.is_dir() {
		return Err(ErrorBadRequest(
			"The folder origin to copy is not a directory.",
		));
	}
	let destiny = Path::new(&destiny);
	copy_dir_all(origin, destiny)?;
	fs::remove_dir_all(origin)?;
	Ok(HttpResponse::Ok().body(format!(
		"Folder moved from: {} to: {}",
		origin.display(),
		destiny.display()
	)))
}

pub fn del(path: PathBuf) -> SrvResult {
	if !path.exists() {
		return Err(ErrorBadRequest("The folder to delete does not exists."));
	}
	if !path.is_dir() {
		return Err(ErrorBadRequest("The folder to delete is not a directory."));
	}
	fs::remove_dir_all(&path)?;
	Ok(HttpResponse::Ok().body(format!("Folder deleted: {}", path.display())))
}
