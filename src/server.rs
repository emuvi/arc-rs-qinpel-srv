use actix_files::NamedFile;
use actix_web::error::Error;
use actix_web::{get, HttpRequest, HttpResponse};

use super::precept;
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

#[get("/shutdown")]
pub async fn shutdown(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    precept::shutdown(&req, &srv_data)
}
