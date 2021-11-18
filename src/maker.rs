use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{
    get, post,
    web::{Bytes, Json},
    HttpRequest,
};

use super::data::RunParams;
use super::guard;
use super::lists;
use super::persist;
use super::precept;
use super::utils;
use super::SrvData;
use super::SrvResult;

#[get("/list/app")]
pub async fn list_app(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_app(&req, &srv_data)
}

#[get("/run/app/*")]
pub async fn run_app(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    guard::check_app_access(&req, &srv_data)?;
    Ok(NamedFile::open(format!("./{}", req.match_info().path()))?)
}

#[get("/list/cmd")]
pub async fn list_cmd(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_cmd(&req, &srv_data)
}

#[post("/run/cmd/*")]
pub async fn run_cmd(
    run_params: Json<RunParams>,
    req: HttpRequest,
    srv_data: SrvData,
) -> SrvResult {
    let req_path = req.match_info().path();
    if req_path.len() < 10 {
        return Err(ErrorBadRequest(
            "Your request must have a bigger command name.",
        ));
    }
    let name = &req.match_info().path()[9..];
    if req_path.starts_with(".") {
        return Err(ErrorBadRequest("The command name can not starts with dot."));
    }
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_cmd_access(name, &user)?;
    precept::run_cmd(name, &run_params, &user, &srv_data.working_dir)
}

#[get("/list/dbs")]
pub async fn list_dbs(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_dbs(&req, &srv_data)
}

#[post("/run/dbs/*")]
pub async fn run_dbs(bytes: Bytes, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let req_path = req.match_info().path();
    if req_path.len() < 10 {
        return Err(ErrorBadRequest(
            "Your request must have a bigger data base source name.",
        ));
    }
    let name = &req.match_info().path()[9..];
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_dbs_access(name, &user)?;
    let name = if name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(name)
    };
    persist::run_dbs(&name, &utils::get_body(bytes)?, &srv_data).await
}

#[post("/ask/dbs/*")]
pub async fn ask_dbs(bytes: Bytes, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let req_path = req.match_info().path();
    if req_path.len() < 10 {
        return Err(ErrorBadRequest(
            "Your request must have a bigger data base source name.",
        ));
    }
    let name = &req.match_info().path()[9..];
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_dbs_access(name, &user)?;
    let name = if name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(name)
    };
    persist::ask_dbs(&name, &utils::get_body(bytes)?, &srv_data).await
}
