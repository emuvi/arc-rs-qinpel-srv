use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::HttpResponse;

use crate::data::Base;
use crate::utils;
use crate::SrvData;
use crate::SrvResult;

pub async fn run_dbs(name: &str, sql: &str, srv_data: &SrvData) -> SrvResult {
	let base = get_base(name, srv_data);
	if base.is_none() {
		let error = format!("could not found the dbs name '{}'", name);
		return Err(ErrorBadRequest(utils::debug(utils::origin!(), &error)));
	}
	let base = base.unwrap();
	let result = srv_data.pooling.run(base, sql).await;
	if let Err(error) = result {
		return Err(ErrorInternalServerError(utils::debug(utils::origin!(), &error)));
	}
	let result = result.unwrap();
	Ok(HttpResponse::Ok().body(format!("Affected: {}", result)))
}

pub async fn ask_dbs(name: &str, sql: &str, srv_data: &SrvData) -> SrvResult {
	let base = get_base(name, srv_data);
	if base.is_none() {
		let error = format!("could not found the dbs name '{}'", name);
		return Err(ErrorBadRequest(utils::debug(utils::origin!(), &error)));
	}
	let base = base.unwrap();
	let result = srv_data.pooling.ask(base, sql).await;
	if let Err(error) = result {
		return Err(ErrorInternalServerError(utils::debug(utils::origin!(), &error)));
	}
	let result = result.unwrap();
	Ok(HttpResponse::Ok().body(result))
}

fn get_base<'a>(name: &'a str, srv_data: &'a SrvData) -> Option<&'a Base> {
	for base in &srv_data.bases {
		if base.name == name {
			return Some(base);
		}
	}
	None
}
