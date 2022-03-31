use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::HttpResponse;
use liz::liz_parse::{self, BlockTrait};
use liz::{liz_codes, liz_forms, liz_texts};
use liz::{liz_dbg_bleb, liz_dbg_errs};
use once_cell::sync::Lazy;

use crate::base::Base;
use crate::srvruns::PathParams;
use crate::SrvData;
use crate::SrvError;
use crate::SrvResult;

pub async fn sql_run(base_name: &str, path_params: &PathParams, srv_data: &SrvData) -> SrvResult {
    let base = get_base(base_name, srv_data)?;
    let source = get_source(path_params)?;
    let result = srv_data.pooling.run(base, &source).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_dbg_errs!(err, base_name)));
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(format!("Affected: {}", result)))
}

pub async fn sql_ask(base_name: &str, path_params: &PathParams, srv_data: &SrvData) -> SrvResult {
    let base = get_base(base_name, srv_data)?;
    let source = get_source(path_params)?;
    let result = srv_data.pooling.ask(base, &source).await;
    if let Err(err) = result {
        return Err(ErrorInternalServerError(liz_dbg_errs!(
            err, base_name, source
        )));
    }
    let result = result.unwrap();
    Ok(HttpResponse::Ok().body(result))
}

fn get_base<'a>(base_name: &str, srv_data: &'a SrvData) -> Result<&'a Base, SrvError> {
    for base in &srv_data.bases {
        if base.name == base_name {
            return Ok(base);
        }
    }
    Err(ErrorBadRequest(liz_dbg_errs!(
        "Could not found the base",
        base_name
    )))
}

static SQL_BLOCKS: Lazy<Vec<Box<dyn BlockTrait>>> = Lazy::new(|| {
    liz::liz_parse::get_parsers(vec![
        liz_parse::block_double_quotes(),
        liz_parse::block_single_quotes(),
        liz_parse::block_white_space(),
        liz_parse::block_char_number('$'),
        liz_parse::block_punctuation(),
    ])
    .map_err(|err| liz_dbg_bleb!(err))
    .expect("Could not get the SQL parser.")
});

fn get_source(path_params: &PathParams) -> Result<String, SrvError> {
    let path = &path_params.path;
    let source = liz_texts::read(path).map_err(|err| ErrorBadRequest(liz_dbg_errs!(err, path)))?;
    let mut code = liz_codes::code(source);
    if let Err(err) = liz_parse::rig_parse_all(&mut code.desk, &SQL_BLOCKS) {
        return Err(ErrorInternalServerError(liz_dbg_errs!(
            err,
            path_params.path
        )));
    }
    if let Some(params) = &path_params.params {
        for (index, param) in params.iter().enumerate() {
            let of = format!("${}", index + 1);
            liz_forms::kit_change_all(&mut code.desk, &of, param);
        }
    }
    Ok(String::default())
}
