use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{web::Bytes, HttpRequest};

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

pub fn get_absolute(path: &str, for_user: &User) -> String {
    fix_absolute(path, &for_user.home)
}

pub fn fix_absolute(path: &str, home: &str) -> String {
    join_paths(home, path)
}

pub fn join_paths(path_a: &str, path_b: &str) -> String {
    let mut result = String::new();
    if starts_with_separator(path_a) {
        result.push(std::path::MAIN_SEPARATOR);
    }
    let mut parts = split_path(path_a);
    let parts_b = split_path(path_b);
    for part_b in parts_b {
        if part_b == "." {
            continue
        } else if part_b == ".." {
            parts.pop();
        } else {
            parts.push(part_b);
        }
    }
    let mut first = true;
    for part in parts {
        if !first {
            result.push(std::path::MAIN_SEPARATOR);
        } else {
            first = false;
        }
        result.push_str(part);
    }
    if ends_with_separator(path_b) {
        result.push(std::path::MAIN_SEPARATOR);
    }
    result
}

pub fn split_path(path: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut start = 0;
    for (i, c) in path.chars().enumerate() {
        if c == '\\' || c == '/'  {
            if i > start {
                result.push(&path[start..i]);
            }
            start = i + 1;
        }
    }
    result
}

pub fn starts_with_separator(path: &str) -> bool {
    path.starts_with("/") || path.starts_with("\\")
}

pub fn ends_with_separator(path: &str) -> bool {
    path.ends_with("/") || path.ends_with("\\")
}
