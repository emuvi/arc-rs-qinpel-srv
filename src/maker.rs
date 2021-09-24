use actix_web::error::ErrorForbidden;
use actix_web::{HttpRequest, HttpResponse};

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

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

pub fn run_cmd(name: &str, run_params: &RunParams, user: &User, working: &str) -> SrvResult {
	let working_dir = Path::new(working);
	let exec_name = format!("{}{}", name, utils::get_exec_extension());
	let full_exec = working_dir.join(&exec_name);
	let full_exec = if !full_exec.exists() {
		working_dir
			.join("run")
			.join("cmd")
			.join(name)
			.join(exec_name)
	} else {
		full_exec
	};
	let mut cmd = Command::new(full_exec);
	cmd.current_dir(working_dir);
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

pub fn run_dbs(name: &str, query: String, srv_data: &SrvData) -> SrvResult {
	Ok(HttpResponse::Ok().body("We don't need to check access here."))
}
