use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::HttpResponse;
use base64;
use liz::liz_dbg_errs;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::SrvResult;

pub fn read(path: &str) -> Result<NamedFile, Error> {
    Ok(NamedFile::open(path)?)
}

pub fn write(path: &str, base64: bool, data: &str) -> SrvResult {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(&path)?;
    write_data(file, base64, data)?;
    Ok(HttpResponse::Ok().body(format!("Written on: {}", path)))
}

pub fn append(path: &str, base64: bool, data: &str) -> SrvResult {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)?;
    write_data(file, base64, data)?;
    Ok(HttpResponse::Ok().body(format!("Appended on: {}", path)))
}

fn write_data(mut file: File, base64: bool, data: &str) -> Result<(), Error> {
    if base64 {
        let bytes = base64::decode(data);
        if let Err(err) = bytes {
            return Err(ErrorBadRequest(liz_dbg_errs!(err)));
        }
        let bytes = bytes.unwrap();
        file.write_all(&bytes)?;
    } else {
        file.write_all(data.as_bytes())?;
    }
    Ok(())
}

pub fn copy(origin: &str, destiny: &str) -> SrvResult {
    let origin_pathed = Path::new(origin);
    if !origin_pathed.exists() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The origin to copy does not exists",
            origin
        )));
    }
    if !origin_pathed.is_file() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The origin to copy is not a file",
            origin
        )));
    }
    std::fs::copy(origin, destiny)?;
    Ok(HttpResponse::Ok().body(format!("File copied from: {} to: {}", origin, destiny)))
}

pub fn mov(origin: &str, destiny: &str) -> SrvResult {
    let origin_pathed = Path::new(origin);
    if !origin_pathed.exists() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The origin to move does not exists",
            origin
        )));
    }
    if !origin_pathed.is_file() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The origin to move is not a file",
            origin
        )));
    }
    std::fs::copy(origin, destiny)?;
    std::fs::remove_file(origin)?;
    Ok(HttpResponse::Ok().body(format!("File moved from: {} to: {}", origin, destiny)))
}

pub fn del(path: &str) -> SrvResult {
    let pathed = Path::new(path);
    if !pathed.exists() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The path to delete does not exists",
            path
        )));
    }
    if !pathed.is_file() {
        return Err(ErrorBadRequest(liz_dbg_errs!(
            "The path to delete is not a file",
            path
        )));
    }
    std::fs::remove_file(&path)?;
    Ok(HttpResponse::Ok().body(format!("File deleted: {}", path)))
}
