use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{web::Bytes};

pub fn get_body(bytes: Bytes) -> Result<String, Error> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(body) => Ok(body),
        Err(utf8_err) => Err(ErrorBadRequest(utf8_err))
    }
}