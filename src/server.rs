use actix_web::error::ErrorNotFound;
use actix_web::{get, HttpRequest, HttpResponse};
use liz::liz_debug;

use super::precept;
use super::SrvData;
use super::SrvResult;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[get("/stop")]
pub async fn stop(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    precept::stop(&req, &srv_data)
}

#[get("/shut")]
pub async fn shut(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    precept::shut(&req, &srv_data)
}

#[get("/version")]
async fn version() -> HttpResponse {
    HttpResponse::Ok().body(format!("{}{}", "v", env!("CARGO_PKG_VERSION")))
}

#[get("*")]
pub async fn redirect(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let path = req.path();
    if let Some(ref redirects) = srv_data.head.redirects {
        if let Some(redirect) = redirects.get(path) {
            return Ok(HttpResponse::Found()
                .header("Location", redirect.clone())
                .finish());
        }
    }
    Err(ErrorNotFound(liz_debug!(
        "Could not found a resource",
        "srv_data.head.redirects",
        path
    )))
}
