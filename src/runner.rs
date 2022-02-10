use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{
    get, post,
    web::{Bytes, Json},
    HttpRequest,
};

use crate::guard;
use crate::lists;
use crate::persist;
use crate::precept::{self, RunParams};
use crate::utils;
use crate::SrvData;
use crate::SrvResult;

#[get("/app/*")]
pub async fn get_app(req: HttpRequest) -> Result<NamedFile, Error> {
    Ok(NamedFile::open(format!("./{}", req.match_info().path()))?)
}

#[post("/cmd/*")]
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

#[post("/run/sql/*")]
pub async fn run_sql(bytes: Bytes, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let req_path = req.match_info().path();
    if req_path.len() < 10 {
        return Err(ErrorBadRequest(
            "Your request must have a bigger data base source name.",
        ));
    }
    let name = &req.match_info().path()[9..];
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_sql_access(name, &user)?;
    let name = if name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(name)
    };
    persist::run_sql(&name, &utils::get_body(bytes)?, &srv_data).await
}

#[post("/ask/sql/*")]
pub async fn ask_sql(bytes: Bytes, req: HttpRequest, srv_data: SrvData) -> SrvResult {
    let req_path = req.match_info().path();
    if req_path.len() < 10 {
        return Err(ErrorBadRequest(
            "Your request must have a bigger data base source name.",
        ));
    }
    let name = &req.match_info().path()[9..];
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_sql_access(name, &user)?;
    let name = if name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(name)
    };
    persist::ask_sql(&name, &utils::get_body(bytes)?, &srv_data).await
}

#[get("/list/apps")]
pub async fn list_apps(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_apps(&req, &srv_data)
}

#[get("/list/cmds")]
pub async fn list_cmds(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_cmds(&req, &srv_data)
}

#[get("/list/sqls")]
pub async fn list_sqls(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_sqls(&req, &srv_data)
}
