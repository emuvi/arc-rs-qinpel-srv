use actix_web::error::{ErrorForbidden};
use actix_web::{HttpRequest, HttpResponse};

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

pub fn run_cmd(cmd_name: &str, run_params: &RunParams, user: &User, working_dir: &str) -> SrvResult {
	let working_dir = Path::new(working_dir).to_owned();
	let exec_name = format!("{}{}", cmd_name, utils::get_exec_extension());
	let full_exec = working_dir.join(&exec_name);
	let present_in_work_dir = full_exec.exists();
	let working_dir = if !present_in_work_dir {
		working_dir.join("run").join("cmd").join(cmd_name)
	} else {
		working_dir
	};
	let full_exec = if !present_in_work_dir {
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
	if run_params.params.len() > 0 {
		for param in &run_params.params {
			cmd.arg(param);
		}
	}
	let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
	if run_params.inputs.len() > 0 {
		let child_stdin = child.stdin.as_mut().unwrap();
		for input in &run_params.inputs {
			child_stdin.write_all(input.as_bytes())?;
		}
	}
	let mut result = String::from("Output: ");
	child.stdout.unwrap().read_to_string(&mut result)?;
	Ok(HttpResponse::Ok().body(result))
}

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