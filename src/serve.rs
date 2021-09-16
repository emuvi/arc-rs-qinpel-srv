use actix_files::NamedFile;
use actix_web::error::Error;
use actix_web::{get, post, web::Bytes, HttpRequest, HttpResponse};

use super::call;
use super::guard;
use super::utils;
use super::SrvData;
use super::SrvResult;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("QinpelSrv pong.")
}

#[get("/favicon.ico")]
pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./favicon.ico")?)
}

#[get("/version")]
async fn version() -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "{}{}",
        "QinpelSrv version: ",
        clap::crate_version!()
    ))
}

#[get("/list/app")]
pub async fn list_app(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    call::list_app(&req, &srv_data)
}

#[get("/run/app/*")]
pub async fn run_app(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    guard::check_run_access(&req, &srv_data)?;
    let file = format!("./{}", req.match_info().path());
    Ok(NamedFile::open(file)?)
}

#[get("/list/cmd")]
pub async fn list_cmd(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    call::list_cmd(&req, &srv_data)
}

#[post("/run/cmd/*")]
pub async fn run_cmd(
    req: HttpRequest,
    srv_data: SrvData,
    bytes: Bytes,
) -> Result<HttpResponse, Error> {
    let body = utils::get_body(bytes)?;
    guard::check_run_access(&req, &srv_data)?;
    Ok(HttpResponse::Ok().body(body))
}

#[get("/list/dbs")]
pub async fn list_dbs(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    call::list_dbs(&req, &srv_data)
}

#[post("/run/dbs/*")]
pub async fn run_dbs(
    req: HttpRequest,
    srv_data: SrvData,
    bytes: Bytes,
) -> Result<HttpResponse, Error> {
    let body = utils::get_body(bytes)?;
    guard::check_run_access(&req, &srv_data)?;
    Ok(HttpResponse::Ok().body(body))
}

#[get("/shutdown")]
pub async fn shutdown(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    guard::check_run_access(&req, &srv_data)?;
    call::shutdown()
}
