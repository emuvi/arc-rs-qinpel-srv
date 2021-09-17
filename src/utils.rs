use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{web::Bytes, HttpRequest};

pub fn get_body(bytes: Bytes) -> Result<String, Error> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(body) => Ok(body),
        Err(utf8_err) => Err(ErrorBadRequest(utf8_err))
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