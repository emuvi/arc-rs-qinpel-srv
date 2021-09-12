use actix_files as actix_fs;
use actix_web::error::{Error};
use actix_web::{get, post, web, web::Bytes, HttpRequest, Responder};

use std::sync::{Arc, RwLock};

use super::call;
use super::data;
use super::guard;
use super::utils;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    "QinpelSrv pong."
}

#[get("/version")]
async fn version() -> impl Responder {
    format!("{}{}", "QinpelSrv version: ", clap::crate_version!())
}

#[get("/shutdown")]
pub async fn shutdown(
    req: HttpRequest,
    srv_data: web::Data<Arc<RwLock<data::Body>>>,
) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::shutdown()
}

#[get("/list/apps")]
pub async fn list_apps() -> Result<impl Responder, Error> {
    call::list_apps()
}

pub fn run_apps() -> actix_fs::Files {
    actix_fs::Files::new("/run/apps", "./run/apps").index_file("index.html")
}

#[get("/list/cmds")]
pub async fn list_cmds(
    req: HttpRequest,
    srv_data: web::Data<Arc<RwLock<data::Body>>>,
) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::list_cmds()
}

#[post("/run/cmds")]
pub async fn run_cmds(
    bytes: Bytes,
    req: HttpRequest,
    srv_data: web::Data<Arc<RwLock<data::Body>>>,
) -> Result<impl Responder, Error> {
    let body = utils::get_body(bytes)?;
    guard::check_access(&req, &srv_data)?;
    Ok(body)
}
