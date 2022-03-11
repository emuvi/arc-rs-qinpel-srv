use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest, ErrorForbidden};
use actix_web::{post, web::Json, HttpRequest};
use liz::{liz_dbg_errs, liz_paths};
use serde::Deserialize;

use super::dirs;
use super::files;
use super::guard;
use super::SrvData;
use super::SrvResult;

#[derive(Deserialize)]
pub struct OnePath {
    pub path: String,
}

#[derive(Deserialize)]
pub struct TwoPath {
    pub origin: String,
    pub destiny: String,
}

#[derive(Deserialize)]
pub struct PathData {
    pub path: String,
    pub base64: bool,
    pub data: String,
}

#[post("/dir/list")]
pub async fn dir_list(one: Json<OnePath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &one.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &one.path)));
        }
    };
    guard::check_dir_access(&path, None, "/dir/list", &user)?;
    dirs::list(&path)
}

#[post("/dir/new")]
pub async fn dir_new(one: Json<OnePath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &one.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &one.path)));
        }
    };
    guard::check_dir_access(&path, None, "/dir/new", &user)?;
    dirs::new(&path)
}

#[post("/dir/copy")]
pub async fn dir_copy(two: Json<TwoPath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let origin = match liz_paths::path_join_if_relative(&user.home, &two.origin) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.origin)));
        }
    };
    let destiny = match liz_paths::path_join_if_relative(&user.home, &two.destiny) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.destiny)));
        }
    };
    guard::check_dir_access(&origin, Some(&destiny), "/dir/copy", &user)?;
    dirs::copy(&origin, &destiny)
}

#[post("/dir/move")]
pub async fn dir_move(two: Json<TwoPath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let origin = match liz_paths::path_join_if_relative(&user.home, &two.origin) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.origin)));
        }
    };
    let destiny = match liz_paths::path_join_if_relative(&user.home, &two.destiny) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.destiny)));
        }
    };
    guard::check_dir_access(&origin, Some(&destiny), "/dir/move", &user)?;
    dirs::mov(&origin, &destiny)
}

#[post("/dir/del")]
pub async fn dir_del(one: Json<OnePath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &one.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &one.path)));
        }
    };
    guard::check_dir_access(&path, None, "/dir/del", &user)?;
    dirs::del(&path)
}

#[post("/file/read")]
pub async fn file_read(
    one: Json<OnePath>,
    req: HttpRequest,
    srv_data: SrvData,
) -> Result<NamedFile, Error> {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &one.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &one.path)));
        }
    };
    guard::check_dir_access(&path, None, "/file/read", &user)?;
    files::read(&path)
}

#[post("/file/write")]
pub async fn file_write(rec: Json<PathData>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &rec.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &rec.path)));
        }
    };
    guard::check_dir_access(&path, None, "/file/write", &user)?;
    files::write(&path, rec.base64, &rec.data)
}

#[post("/file/append")]
pub async fn file_append(rec: Json<PathData>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &rec.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &rec.path)));
        }
    };
    guard::check_dir_access(&path, None, "/file/append", &user)?;
    files::append(&path, rec.base64, &rec.data)
}

#[post("/file/copy")]
pub async fn file_copy(two: Json<TwoPath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let origin = match liz_paths::path_join_if_relative(&user.home, &two.origin) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.origin)));
        }
    };
    let destiny = match liz_paths::path_join_if_relative(&user.home, &two.destiny) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.destiny)));
        }
    };
    guard::check_dir_access(&origin, Some(&destiny), "/file/copy", &user)?;
    files::copy(&origin, &destiny)
}

#[post("/file/move")]
pub async fn file_move(two: Json<TwoPath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let origin = match liz_paths::path_join_if_relative(&user.home, &two.origin) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.origin)));
        }
    };
    let destiny = match liz_paths::path_join_if_relative(&user.home, &two.destiny) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &two.destiny)));
        }
    };
    guard::check_dir_access(&origin, Some(&destiny), "/file/move", &user)?;
    files::mov(&origin, &destiny)
}

#[post("/file/del")]
pub async fn file_del(one: Json<OnePath>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource.",
        ));
    }
    let user = user.unwrap();
    let path = match liz_paths::path_join_if_relative(&user.home, &one.path) {
        Ok(path) => path,
        Err(err) => {
            return Err(ErrorBadRequest(liz_dbg_errs!(err, &user.home, &one.path)));
        }
    };
    guard::check_dir_access(&path, None, "/file/del", &user)?;
    files::del(&path)
}
