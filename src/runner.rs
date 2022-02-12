use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{
    get, post,
    web::{Json, Path},
    HttpRequest,
};
use liz::liz_paths;
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
    let wd = &srv_data.working_dir;
    let req_path = format!(".{}", req.match_info().path());
    let file_path = liz_paths::path_join(wd, &req_path).map_err(|err| {
        ErrorBadRequest(format!(
            "Could not get the public file at {} because {}",
            req_path, err
        ))
    })?;
    Ok(NamedFile::open(file_path)?)
}

#[get("/app/{name}/*")]
pub async fn get_app(
    req: HttpRequest,
    name: Path<String>,
    srv_data: SrvData,
) -> Result<NamedFile, Error> {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_app_access(name.as_ref(), user)?;
    let wd = &srv_data.working_dir;
    let req_path = format!(".{}", req.match_info().path());
    let file_path = liz_paths::path_join(wd, &req_path).map_err(|err| {
        ErrorBadRequest(format!(
            "Could not get the application file at {} because {}",
            req_path, err
        ))
    })?;
    Ok(NamedFile::open(file_path)?)
}

#[get("/list/apps")]
pub async fn list_apps(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_apps(&req, &srv_data)
}

#[post("/cmd/{name}")]
pub async fn run_cmd(
    req: HttpRequest,
    name: Path<String>,
    args_inputs: Json<ArgsInputs>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_cmd_access(name.as_ref(), user)?;
    precept::run_cmd(name.as_ref(), &args_inputs, &user, &srv_data.working_dir)
}

#[get("/list/cmds")]
pub async fn list_cmds(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    lists::list_cmds(&req, &srv_data)
}

#[post("/run/sql/{bas_name}")]
pub async fn run_sql(
    req: HttpRequest,
    bas_name: Path<String>,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_sql_access(&bas_name, &path_params.path, &user)?;
    let bas_name: String = if bas_name.as_ref() == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(bas_name.as_ref())
    };
    persist::run_sql(&bas_name, &path_params, &srv_data).await
}

#[post("/ask/sql/{bas_name}")]
pub async fn ask_sql(
    req: HttpRequest,
    bas_name: Path<String>,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    let user = guard::get_user_or_err(&req, &srv_data)?;
    guard::check_sql_access(&bas_name, &path_params.path, &user)?;
    let bas_name: String = if bas_name.as_ref() == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(bas_name.as_ref())
    };
    persist::ask_sql(&bas_name, &path_params, &srv_data).await
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
