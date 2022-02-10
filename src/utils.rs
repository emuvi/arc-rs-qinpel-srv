use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{web::Bytes, HttpRequest};

use std::fmt::Display;
use std::marker::Sized;

macro_rules! origin {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

pub(crate) use origin;

pub fn debug<TD: Display + ?Sized>(origin: &'static str, error: &TD) -> String {
    let message = format!("Problem on {} - {}", origin, error);
    eprintln!("{}", message);
    message
}

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

