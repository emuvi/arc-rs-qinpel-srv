use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::HttpResponse;
use liz::{liz_debug, liz_texts};

use crate::data::Base;
use crate::runner::PathParams;
use crate::SrvData;
use crate::SrvError;
use crate::SrvResult;

pub async fn run_sql(bas_name: &str, path_params: &PathParams, srv_data: &SrvData) -> SrvResult {
    let base = get_base(bas_name, srv_data)?;
    let source = get_source(path_params)?;
    let result = srv_data.pooling.run(base, &source).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_debug!(
            err, "run_sql", bas_name, source
        ))); // TODO - evaluate if we should use the liz debug in all places in the QinpelSrv
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(format!("Affected: {}", result)))
}

pub async fn ask_sql(bas_name: &str, path_params: &PathParams, srv_data: &SrvData) -> SrvResult {
    let base = get_base(bas_name, srv_data)?;
    let source = get_source(path_params)?;
    let result = srv_data.pooling.ask(base, &source).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_debug!(
            err, "ask_sql", bas_name, source
        )));
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(result))
}

fn get_base<'a>(bas_name: &str, srv_data: &'a SrvData) -> Result<&'a Base, SrvError> {
    for base in &srv_data.bases {
        if base.name == bas_name {
            return Ok(base);
        }
    }
    Err(ErrorBadRequest(format!(
        "Could not found the base with the name = '{}'",
        bas_name
    )))
}

fn get_source(path_params: &PathParams) -> Result<String, SrvError> {
    let path = &path_params.path;
    let source =
        liz_texts::read(path).map_err(|err| ErrorBadRequest(liz_debug!(err, "read", path)))?;
    // TODO string interpolation - use the liz source feature to do
    Ok(String::default())
}
