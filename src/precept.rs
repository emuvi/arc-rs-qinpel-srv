use actix_web::error::{ErrorForbidden, ErrorInternalServerError};
use actix_web::{HttpRequest, HttpResponse};
use futures::executor;
use liz::{self, liz_dbg_errs, liz_fires};

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::data::Access;
use crate::data::User;
use crate::guard;
use crate::runner::{ArgsInputs, PathParams};
use crate::SrvData;
use crate::SrvResult;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(1000);

pub fn shut(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
    if let Some(user) = guard::get_user(req, srv_data) {
        if user.master {
            let name = &srv_data.head.server_name;
            let result = format!("{} is shutting...", name);
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

pub fn stop(req: &HttpRequest, srv_data: &SrvData) -> SrvResult {
    if let Some(user) = guard::get_user(req, srv_data) {
        if user.master {
            let data_server = srv_data.server.read().unwrap();
            if let Some(server) = &*data_server {
                let name = &srv_data.head.server_name;
                let result = format!("{} is stopping...", name);
                println!("{}", result);
                executor::block_on(server.stop(false));
                return Ok(HttpResponse::Ok().body(result));
            }
        }
    }
    Err(ErrorForbidden(
        "You don't have access to call this resource.",
    ))
}

pub fn run_cmd(
    cmd_name: &str,
    args_inputs: &ArgsInputs,
    user: &User,
    working_dir: &str,
) -> SrvResult {
    let working_dir = Path::new(working_dir).to_owned();
    let exec_name = format!("{}{}", cmd_name, liz_fires::exe_ext());
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
        if let Access::CMD { name, fixed_args } = an_access {
            if name == cmd_name {
                for arg in fixed_args {
                    cmd.arg(arg);
                }
            }
        }
    }
    if let Some(args) = &args_inputs.args {
        for arg in args {
            cmd.arg(arg);
        }
    }
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    if let Some(inputs) = &args_inputs.inputs {
        let child_stdin = child.stdin.as_mut().unwrap();
        for input in inputs {
            child_stdin.write_all(input.as_bytes())?;
            child_stdin.write_all("\n".as_bytes())?;
        }
    }
    let mut result = String::from("Output: ");
    child.stdout.unwrap().read_to_string(&mut result)?;
    Ok(HttpResponse::Ok().body(result))
}

pub fn run_liz(path_params: &PathParams) -> SrvResult {
    let results = liz::run(&path_params.path, &path_params.params)
        .map_err(|err| ErrorInternalServerError(liz_dbg_errs!(err, path_params)))?;
    let mut body = String::from("[");
    let mut first = true;
    for result in results {
        if first {
            first = false;
        } else {
            body.push(',');
        }
        body.push_str(&result);
    }
    body.push(']');
    Ok(HttpResponse::Ok().body(body))
}
