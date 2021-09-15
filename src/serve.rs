use actix_files::NamedFile;
use actix_web::error::Error;
use actix_web::{get, post, web::Bytes, HttpRequest, Responder};

use super::call;
use super::guard;
use super::utils;
use super::SrvData;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    "QinpelSrv pong."
}

#[get("/favicon.ico")]
pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./favicon.ico")?)
}

#[get("/version")]
async fn version() -> impl Responder {
    format!("{}{}", "QinpelSrv version: ", clap::crate_version!())
}

#[get("/list/app")]
pub async fn list_app(req: HttpRequest, srv_data: SrvData) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::list_app()
}

#[get("/run/app/*")]
pub async fn run_app(req: HttpRequest, srv_data: SrvData) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    let file = format!("./{}", req.match_info().path());
    Ok(NamedFile::open(file)?)
}

#[get("/list/cmd")]
pub async fn list_cmd(req: HttpRequest, srv_data: SrvData) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::list_cmd()
}

#[post("/run/cmd/*")]
pub async fn run_cmd(
    req: HttpRequest,
    srv_data: SrvData,
    bytes: Bytes,
) -> Result<impl Responder, Error> {
    let body = utils::get_body(bytes)?;
    guard::check_access(&req, &srv_data)?;
    Ok(body)
}

#[get("/list/dbs")]
pub async fn list_dbs(req: HttpRequest, srv_data: SrvData) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::list_cmd()
}

#[post("/run/dbs/*")]
pub async fn run_dbs(
    req: HttpRequest,
    srv_data: SrvData,
    bytes: Bytes,
) -> Result<impl Responder, Error> {
    let body = utils::get_body(bytes)?;
    guard::check_access(&req, &srv_data)?;
    Ok(body)
}

#[get("/shutdown")]
pub async fn shutdown(req: HttpRequest, srv_data: SrvData) -> Result<impl Responder, Error> {
    guard::check_access(&req, &srv_data)?;
    call::shutdown()
}