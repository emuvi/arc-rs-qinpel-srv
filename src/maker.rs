use actix_web::error::{ErrorBadRequest, ErrorForbidden, ErrorServiceUnavailable};
use actix_web::{HttpRequest, HttpResponse};
use r2d2_sqlite::rusqlite::types::ValueRef;
use rust_decimal::Decimal;

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use super::data::Access;
use super::data::RunParams;
use super::data::User;
use super::guard;
use super::utils;
use super::SrvData;
use super::SrvResult;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(3000);

pub fn shutdown(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
	if let Some(user) = guard::get_user(req, srv_data) {
		if user.master {
			let result = String::from("QinpelSrv is shutdown...");
			println!("{}", result);
			std::thread::spawn(|| {
				std::thread::sleep(SLEEP_TO_SHUTDOWN);
				std::process::exit(0);
			});
			return Ok(HttpResponse::Ok().body(result));
		}
	}
	Err(ErrorForbidden(
		"You don't have access to call this resource.",
	))
}

pub fn run_cmd(cmd_name: &str, run_params: &RunParams, user: &User, srv_desk: &str) -> SrvResult {
	let working_dir = Path::new(srv_desk).to_owned();
	let exec_name = format!("{}{}", cmd_name, utils::get_exec_extension());
	let full_exec = working_dir.join(&exec_name);
	let working_dir = if !full_exec.exists() {
		working_dir.join("run").join("cmd").join(cmd_name)
	} else {
		working_dir
	};
	let full_exec = if !full_exec.exists() {
		working_dir.join(exec_name)
	} else {
		full_exec
	};
	let mut cmd = Command::new(full_exec);
	cmd.current_dir(working_dir);
	for an_access in &user.access {
		if let Access::CMD { name, params } = an_access {
			if name == cmd_name {
				for param in params {
					cmd.arg(param);
				}
			}
		}
	}
	if let Some(params) = &run_params.params {
		for param in params {
			cmd.arg(param);
		}
	}
	let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
	if let Some(inputs) = &run_params.inputs {
		let child_stdin = child.stdin.as_mut().unwrap();
		for input in inputs {
			child_stdin.write_all(input.as_bytes())?;
		}
		drop(child_stdin);
	}
	let mut result = String::from("Output: ");
	child.stdout.unwrap().read_to_string(&mut result)?;
	Ok(HttpResponse::Ok().body(result))
}

pub fn run_dbs(name: &str, sql: String, srv_data: &SrvData) -> SrvResult {
	if let Some(pool) = srv_data.bases_sqlite.get(name) {
		let pool = pool.clone();
		let conn = pool.get();
		if let Err(error) = conn {
			return Err(ErrorServiceUnavailable(format!(
				"Could not get the connection for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let conn = conn.unwrap();
		let result = conn.execute(&sql, []);
		if let Err(error) = result {
			return Err(ErrorServiceUnavailable(format!(
				"Could not execute the query for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let result = result.unwrap();
		return Ok(HttpResponse::Ok().body(format!("Execution success with affected: {}", result)));
	} else if let Some(pool) = srv_data.bases_postgres.get(name) {
		let pool = pool.clone();
		let conn = pool.get();
		if let Err(error) = conn {
			return Err(ErrorServiceUnavailable(format!(
				"Could not get the connection for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let mut conn = conn.unwrap();
		let ref_sql: &str = sql.as_ref();
		let result = conn.execute(ref_sql, &[]);
		if let Err(error) = result {
			return Err(ErrorServiceUnavailable(format!(
				"Could not execute the query for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let result = result.unwrap();
		return Ok(HttpResponse::Ok().body(format!("Execution success with affected: {}", result)));
	}
	Err(ErrorBadRequest(format!(
		"Could not found the base source with the name: {}.",
		name
	)))
}

pub fn ask_dbs(name: &str, sql: String, srv_data: &SrvData) -> SrvResult {
	if let Some(pool) = srv_data.bases_sqlite.get(name) {
		let pool = pool.clone();
		let conn = pool.get();
		if let Err(error) = conn {
			return Err(ErrorServiceUnavailable(format!(
				"Could not get the connection for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let conn = conn.unwrap();
		let mut stmt = conn.prepare(&sql).unwrap();
		let mut rows = stmt.query([]).unwrap();
		let column_count = rows.column_count().unwrap();
		let mut result = String::new();
		while let Some(row) = rows.next().unwrap() {
			for column_index in 0..column_count {
				let column_value = if let Ok(ref_value) = row.get_ref(column_index) {
					match ref_value {
						ValueRef::Null => String::new(),
						ValueRef::Integer(value) => value.to_string(),
						ValueRef::Real(value) => value.to_string(),
						ValueRef::Text(value) => String::from_utf8(value.to_vec()).unwrap(),
						ValueRef::Blob(_) => String::from("<err>"),
					}
				} else {
					String::from("<err>")
				};
				let column_value = get_column_value_for_csv(column_value);
				if column_index > 0 {
					result.push(',');
				}
				result.push_str(&column_value);
			}
			result.push('\n');
		}
		return Ok(HttpResponse::Ok().body(result));
	} else if let Some(pool) = srv_data.bases_postgres.get(name) {
		let pool = pool.clone();
		let conn = pool.get();
		if let Err(error) = conn {
			return Err(ErrorServiceUnavailable(format!(
				"Could not get the connection for the base source name: {}. - Error: {}",
				name, error
			)));
		}
		let mut conn = conn.unwrap();
		let ref_sql: &str = sql.as_ref();
		let rows = conn.query(ref_sql, &[]).unwrap();
		let mut result = String::new();
		for row in rows {
			let columns = row.columns();
			let column_count = columns.len();
			for column_index in 0..column_count {
				let column = &columns[column_index];
				let column_type = column.type_().name();
				let column_value = match column_type {
					"bool" => {
						let value: bool = row.get(column_index);
						value.to_string()
					}
					"char" => {
						let value: i8 = row.get(column_index);
						value.to_string()
					}
					"int2" => {
						let value: i16 = row.get(column_index);
						value.to_string()
					}
					"int4" => {
						let value: i32 = row.get(column_index);
						value.to_string()
					}
					"oid" => {
						let value: u32 = row.get(column_index);
						value.to_string()
					}
					"int8" => {
						let value: i64 = row.get(column_index);
						value.to_string()
					}
					"float4" => {
						let value: f32 = row.get(column_index);
						value.to_string()
					}
					"float8" => {
						let value: f64 = row.get(column_index);
						value.to_string()
					}
					"numeric" => {
						let value: Decimal = row.get(column_index);
						value.to_string()
					}
					"varchar" | "_char" | "name" | "text" => {
						let value: String = row.get(column_index);
						value
					}
					_ => String::from("<err>"),
				};
				let column_value = get_column_value_for_csv(column_value);
				if column_index > 0 {
					result.push(',');
				}
				result.push_str(&column_value);
			}
			result.push('\n');
		}
		return Ok(HttpResponse::Ok().body(result));
	}
	Err(ErrorBadRequest(format!(
		"Could not found the base source with the name: {}.",
		name
	)))
}

fn get_column_value_for_csv(column_value: String) -> String {
	let mut result = column_value
		.replace('"', "\"\"")
		.replace('\\', "\\\\")
		.replace("\r", "\\r")
		.replace("\n", "\\n")
		.replace("\t", "\\t");
	if result.contains('"') || result.contains(",") {
		result.insert(0, '"');
		result.push('"');
	}
	result
}
