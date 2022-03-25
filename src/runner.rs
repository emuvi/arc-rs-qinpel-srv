use actix_files::NamedFile;
use actix_web::error::{Error, ErrorBadRequest};
use actix_web::{get, post, web::Json, HttpRequest};
use liz::{liz_dbg_call, liz_dbg_errs, liz_dbg_reav, liz_dbg_step, liz_paths};
use serde::Deserialize;

use crate::bad_req;
use crate::guard;
use crate::lists;
use crate::persist;
use crate::precept;
use crate::SrvData;
use crate::SrvResult;

#[get("/pub/*")]
pub async fn get_pub(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    liz_dbg_call!(req, srv_data);
    let srv_dir = &srv_data.srv_dir;
    liz_dbg_step!(srv_dir);
    let req_path = format!(".{}", req.match_info().path());
    liz_dbg_step!(req_path);
    let file_path = liz_paths::path_join(srv_dir, &req_path).map_err(|err| bad_req(err))?;
    liz_dbg_step!(file_path);
    liz_dbg_reav!(Ok(NamedFile::open(file_path)?))
}

#[get("/app/*")]
pub async fn get_app(req: HttpRequest, srv_data: SrvData) -> Result<NamedFile, Error> {
    liz_dbg_call!(req, srv_data);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let app_name = path
        .split("/")
        .nth(2)
        .ok_or("Could not found the application name")
        .map_err(|err| ErrorBadRequest(liz_dbg_errs!(err, path)))?;
    liz_dbg_step!(path);
    if app_name != "qinpel-app" {
        let user = guard::get_user_or_err(&req, &srv_data)?;
        liz_dbg_step!(user);
        guard::check_app_access(app_name, user)?;
    }
    let srv_dir = &srv_data.srv_dir;
    liz_dbg_step!(srv_dir);
    let req_path = format!(".{}", req.match_info().path());
    liz_dbg_step!(req_path);
    let file_path = liz_paths::path_join(srv_dir, &req_path).map_err(|err| bad_req(err))?;
    liz_dbg_reav!(Ok(NamedFile::open(file_path)?));
}

#[get("/list/apps")]
pub async fn list_apps(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    liz_dbg_reav!(lists::list_apps(&req, &srv_data));
}

#[post("/cmd/*")]
pub async fn run_cmd(
    req: HttpRequest,
    args_inputs: Json<ArgsInputs>,
    srv_data: SrvData,
) -> SrvResult {
    liz_dbg_call!(req, args_inputs, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let cmd_name = path
        .split("/")
        .nth(2)
        .ok_or("Could not found the command name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(cmd_name);
    guard::check_cmd_access(cmd_name, user)?;
    liz_dbg_reav!(precept::run_cmd(cmd_name, &args_inputs, &user, &srv_data.srv_dir))
}

#[get("/list/cmds")]
pub async fn list_cmds(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    liz_dbg_reav!(lists::list_cmds(&req, &srv_data));
}

#[post("/dbs/new/*")]
pub async fn dbs_new(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(base_name);
    Ok("".into())
}

#[post("/dbs/ask/*")]
pub async fn dbs_ask(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(base_name);
    Ok("".into())
}

#[post("/dbs/set/*")]
pub async fn dbs_set(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(base_name);
    Ok("".into())
}

#[post("/dbs/del/*")]
pub async fn dbs_del(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(base_name);
    Ok("".into())
}


#[post("/run/sql/*")]
pub async fn run_sql(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    liz_dbg_call!(req, path_params, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| bad_req(err))?;
    liz_dbg_step!(base_name);
    guard::check_sql_access(&base_name, &path_params.path, &user)?;
    let base_name = if base_name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(base_name)
    };
    liz_dbg_step!(base_name);
    liz_dbg_reav!(persist::run_sql(&base_name, &path_params, &srv_data).await);
}

#[post("/ask/sql/*")]
pub async fn ask_sql(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    liz_dbg_call!(req, path_params, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    let path = req.match_info().path();
    liz_dbg_step!(path);
    let base_name = path
        .split("/")
        .nth(3)
        .ok_or("Could not found the data base name")
        .map_err(|err| ErrorBadRequest(liz_dbg_errs!(err, path)))?;
    liz_dbg_step!(base_name);
    guard::check_sql_access(&base_name, &path_params.path, &user)?;
    let base_name = if base_name == "default_dbs" {
        format!("{}_default_dbs", user.name)
    } else {
        String::from(base_name)
    };
    liz_dbg_step!(base_name);
    liz_dbg_reav!(persist::ask_sql(&base_name, &path_params, &srv_data).await);
}

#[get("/list/bases")]
pub async fn list_bases(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    liz_dbg_reav!(lists::list_bases(&req, &srv_data));
}

#[post("/run/liz")]
pub async fn run_liz(
    req: HttpRequest,
    path_params: Json<PathParams>,
    srv_data: SrvData,
) -> SrvResult {
    liz_dbg_call!(req, path_params, srv_data);
    let user = guard::get_user_or_err(&req, &srv_data)?;
    liz_dbg_step!(user);
    guard::check_liz_access(&path_params.path, &user)?;
    liz_dbg_reav!(precept::run_liz(&path_params));
}

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
