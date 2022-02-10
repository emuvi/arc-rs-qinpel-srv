use actix_web::error::ErrorBadRequest;
use actix_web::HttpResponse;
use liz::liz_debug;

use std::path::Path;
use crate::SrvResult;

pub fn list(path: &str) -> SrvResult {
    let pathed = Path::new(path);
    if !pathed.exists() {
        return Err(ErrorBadRequest(liz_debug!(
            "The path to list does not exists",
            "exists",
            path
        )));
    }
    if !pathed.is_dir() {
        return Err(ErrorBadRequest(liz_debug!(
            "The path to list is not a directory",
            "is_dir",
            path
        )));
    }
    let mut body = String::new();
    body.push_str("P: ");
    body.push_str(path);
    body.push_str("\n");
    for entry in pathed.read_dir()? {
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

pub fn new(path: &str) -> SrvResult {
    std::fs::create_dir_all(&path)?;
    Ok(HttpResponse::Ok().body(format!("Folder created: {}", path)))
}

pub fn copy(origin: &str, destiny: &str) -> SrvResult {
    let origin_pathed = Path::new(origin);
    let destiny_pathed = Path::new(destiny);
    if !origin_pathed.exists() {
        return Err(ErrorBadRequest(liz_debug!(
            "The origin to copy does not exists",
            "exists",
            origin
        )));
    }
    if !origin_pathed.is_dir() {
        return Err(ErrorBadRequest(liz_debug!(
            "The origin to copy is not a directory",
            "is_dir",
            origin
        )));
    }
    copy_dir_all(&origin, &destiny)?;
    Ok(HttpResponse::Ok().body(format!("Folder copied from: {} to: {}", origin, destiny)))
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn mov(origin: &str, destiny: &str) -> SrvResult {
    let origin_pathed = Path::new(origin);
    let destiny_pathed = Path::new(destiny);
    if !origin_pathed.exists() {
        return Err(ErrorBadRequest(liz_debug!(
            "The origin to move does not exists",
            "exists",
            origin
        )));
    }
    if !origin_pathed.is_dir() {
        return Err(ErrorBadRequest(liz_debug!(
            "The origin to move is not a directory",
            "is_dir",
            origin
        )));
    }
    copy_dir_all(origin, destiny)?;
    std::fs::remove_dir_all(origin)?;
    Ok(HttpResponse::Ok().body(format!("Folder moved from: {} to: {}", origin, destiny)))
}

pub fn del(path: &str) -> SrvResult {
    let pathed = Path::new(path);
    if !pathed.exists() {
        return Err(ErrorBadRequest(liz_debug!(
            "The path to delete does not exists",
            "exists"
        )));
    }
    if !pathed.is_dir() {
        return Err(ErrorBadRequest(liz_debug!(
            "The path to delete is not a directory",
            "is_dir"
        )));
    }
    std::fs::remove_dir_all(path)?;
    Ok(HttpResponse::Ok().body(format!("Folder deleted: {}", path)))
}
