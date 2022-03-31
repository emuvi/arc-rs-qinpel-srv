use actix_web::error::ErrorBadRequest;
use actix_web::{get, post, web::Json, HttpRequest};
use liz::{liz_dbg_call, liz_dbg_errs, liz_dbg_reav, liz_dbg_step};

use crate::bad_req;
use crate::guard;
use crate::lists;
use crate::persist;
use crate::srvruns::PathParams;
use crate::SrvData;
use crate::SrvResult;

#[post("/reg/new/*")]
pub async fn reg_new(req: HttpRequest, srv_data: SrvData) -> SrvResult {
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

#[post("/reg/ask/*")]
pub async fn reg_ask(req: HttpRequest, srv_data: SrvData) -> SrvResult {
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

#[post("/reg/set/*")]
pub async fn reg_set(req: HttpRequest, srv_data: SrvData) -> SrvResult {
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

#[post("/reg/del/*")]
pub async fn reg_del(req: HttpRequest, srv_data: SrvData) -> SrvResult {
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

#[post("/sql/run/*")]
pub async fn sql_run(
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
    liz_dbg_reav!(persist::sql_run(&base_name, &path_params, &srv_data).await);
}

#[post("/sql/ask/*")]
pub async fn sql_ask(
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
    liz_dbg_reav!(persist::sql_ask(&base_name, &path_params, &srv_data).await);
}

#[get("/list/bases")]
pub async fn list_bases(req: HttpRequest, srv_data: SrvData) -> SrvResult {
    liz_dbg_call!(req, srv_data);
    liz_dbg_reav!(lists::list_bases(&req, &srv_data));
}
