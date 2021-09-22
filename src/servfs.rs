use actix_web::error::{ErrorForbidden};
use actix_web::{post, web, HttpRequest};
use serde_derive::Deserialize;

use super::dirs;
use super::files;
use super::guard;
use super::utils;
use super::SrvData;
use super::SrvResult;

#[derive(Deserialize)]
pub struct One {
    pub path: String,
}

#[derive(Deserialize)]
pub struct Two {
    pub origin: String,
    pub destiny: String,
}

#[post("/dir/list")]
pub async fn dir_list(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/dir/list", &user)?;
    dirs::list(path)
}

#[post("/dir/new")]
pub async fn dir_new(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/dir/new", &user)?;
    dirs::new(path)
}

#[post("/dir/copy")]
pub async fn dir_copy(two: web::Json<Two>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let origin = utils::get_absolute(&two.origin, &user);
    let destiny = utils::get_absolute(&two.destiny, &user);
    guard::check_dir_access(&origin, Some(&destiny), "/dir/copy", &user)?;
    dirs::copy(origin, destiny)
}

#[post("/dir/move")]
pub async fn dir_move(two: web::Json<Two>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let origin = utils::get_absolute(&two.origin, &user);
    let destiny = utils::get_absolute(&two.destiny, &user);
    guard::check_dir_access(&origin, Some(&destiny), "/dir/move", &user)?;
    dirs::mov(origin, destiny)
}

#[post("/dir/del")]
pub async fn dir_del(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/dir/del", &user)?;
    dirs::del(path)
}

#[post("/file/read")]
pub async fn file_read(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/file/read", &user)?;
    files::read(path)
}

#[post("/file/write")]
pub async fn file_write(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/file/write", &user)?;
    files::write(path)
}

#[post("/file/append")]
pub async fn file_append(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/file/append", &user)?;
    files::append(path)
}

#[post("/file/copy")]
pub async fn file_copy(two: web::Json<Two>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let origin = utils::get_absolute(&two.origin, &user);
    let destiny = utils::get_absolute(&two.destiny, &user);
    guard::check_dir_access(&origin, Some(&destiny), "/file/copy", &user)?;
    files::copy(origin, destiny)
}

#[post("/file/move")]
pub async fn file_move(two: web::Json<Two>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let origin = utils::get_absolute(&two.origin, &user);
    let destiny = utils::get_absolute(&two.destiny, &user);
    guard::check_dir_access(&origin, Some(&destiny), "/file/move", &user)?;
    files::mov(origin, destiny)
}

#[post("/file/del")]
pub async fn file_del(one: web::Json<One>, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let user = guard::get_user(&req, &srv_data);
    if user.is_none() {
        return Err(ErrorForbidden("You don't have access to call this resource."));
    }
    let user = user.unwrap();
    let path = utils::get_absolute(&one.path, &user);
    guard::check_dir_access(&path, None, "/file/del", &user)?;
    files::del(path)
}
