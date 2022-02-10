use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::HttpResponse;
use liz::liz_debug;

use crate::data::Base;
use crate::SrvData;
use crate::SrvError;
use crate::SrvResult;

pub async fn run_sql(name: &str, sql: &str, srv_data: &SrvData) -> SrvResult {
    let base = get_base(name, srv_data)?;
    let result = srv_data.pooling.run(base, sql).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_debug!(
            err, "run_sql", name, sql
        )));
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(format!("Affected: {}", result)))
}

pub async fn ask_sql(name: &str, sql: &str, srv_data: &SrvData) -> SrvResult {
    let base = get_base(name, srv_data)?;
    let result = srv_data.pooling.ask(base, sql).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_debug!(
            err, "ask_sql", name, sql
        )));
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(result))
}

fn get_base<'a>(name: &'a str, srv_data: &'a SrvData) -> Result<&'a Base, SrvError> {
    for base in &srv_data.bases {
        if base.name == name {
            return Ok(base);
        }
    }
    Err(ErrorBadRequest(format!(
        "Could not found the base with the name = '{}'",
        name
    )))
}
