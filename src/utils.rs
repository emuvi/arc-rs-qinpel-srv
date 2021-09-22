use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{web::Bytes, HttpRequest};

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::data::User;

pub fn get_body(bytes: Bytes) -> Result<String, Error> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(body) => Ok(body),
        Err(utf8_err) => Err(ErrorBadRequest(utf8_err)),
    }
}

pub fn get_lang(req: &HttpRequest) -> String {
    if let Some(lang) = req.headers().get("Accept-Language") {
        if let Ok(lang) = lang.to_str() {
            return String::from(lang);
        }
    }
    String::from("en")
}

pub fn get_absolute(path: &String, for_user: &User) -> PathBuf {
    Path::new(path).to_owned()
}

pub fn fix_absolute(path: &str) -> String {
    if let Ok(fixed) = fs::canonicalize(path) {
        return format!("{}", fixed.display());
    }
    String::from(path)
}
