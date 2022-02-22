use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{get, post, web::Json, HttpRequest};
use liz::{liz_debug, liz_paths};
use serde::Deserialize;

use crate::guard;
use crate::lists;
use crate::persist;
use crate::precept;
use crate::SrvData;
use crate::SrvResult;

#[derive(Debug, Deserialize)]
pub struct ArgsInputs {
    pub args: Option<Vec<String>>,
    pub inputs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PathParams {
    pub path: String,
    pub params: Option<Vec<String>>,
}

#[get("/pub/*")]
pub async fn get_pub(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    let working_dir = &srv_data.working_dir;
    let req_path = format!(".{}", req.match_info().path());
    let file_path = liz_paths::path_join(working_dir, &req_path)
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "path_join", working_dir, req_path)))?;
    Ok(NamedFile::open(file_path)?)
}

#[get("/app/*")]
pub async fn get_app(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    let path = req.match_info().path();
    let app_name = path
        .split("/")
        .nth(2)
        .ok_or("Could not found the app name")
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "split", path)))?;
    if app_name != "qinpel-app" {
        let user = guard::get_user_or_err(&req, &srv_data)?;
        guard::check_app_access(app_name, user)?;
    }
    let working_dir = &srv_data.working_dir;
    let req_path = format!(".{}", req.match_info().path());
    let file_path = liz_paths::path_join(working_dir, &req_path)
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "path_join", working_dir, req_path)))?;
    Ok(NamedFile::open(file_path)?)
}

#[get("/list/apps")]
pub async fn list_apps(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_apps(&req, &srv_data)
}

#[post("/cmd/*")]
pub async fn run_cmd(
    req: HttpRequest,
    args_inputs: Json<ArgsInputs>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    let path = req.match_info().path();
    let cmd_name = path
        .split("/")
        .nth(2)
        .ok_or("Could not found the cmd name")
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "split", path)))?;
    guard::check_cmd_access(cmd_name, user)?;
    precept::run_cmd(cmd_name, &args_inputs, &user, &srv_data.working_dir)
}

#[get("/list/cmds")]
pub async fn list_cmds(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_cmds(&req, &srv_data)
}

#[post("/run/sql/*")]
pub async fn run_sql(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    let path = req.match_info().path();
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the cmd name")
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "split", path)))?;
    
    guard::check_sql_access(&base_name, &path_params.path, &user)?;
    let base_name = if base_name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(base_name)
    };
    persist::run_sql(&base_name, &path_params, &srv_data).await
}

#[post("/ask/sql/*")]
pub async fn ask_sql(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    let path = req.match_info().path();
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the cmd name")
        .map_err(|err| ErrorBadRequest(liz_debug!(err, "split", path)))?;
    
    guard::check_sql_access(&base_name, &path_params.path, &user)?;
    let base_name = if base_name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(base_name)
    };
    persist::ask_sql(&base_name, &path_params, &srv_data).await
}

#[get("/list/bases")]
pub async fn list_bases(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_bases(&req, &srv_data)
}

#[post("/run/liz")]
pub async fn run_liz(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_liz_access(&path_params.path, &user)?;
    precept::run_liz(&path_params)
}
