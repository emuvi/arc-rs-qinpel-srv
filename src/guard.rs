use actix_web::error::{Error, ErrorForbidden};
use actix_web::HttpRequest;
use liz::liz_debug;

use crate::data::Access;
use crate::data::User;
use crate::data::Base;
use crate::SrvData;

pub fn get_user<'a>(req: &HttpRequest, srv_data: &'a SrvData) -> Option<&'a User> {
    if is_debug_local(req, srv_data) {
        let users = &srv_data.users;
        if let Some(root) = users.iter().find(|user| user.name == "root") {
            return Some(root);
        }
    }
    get_token_user(req, srv_data)
}

pub fn get_user_or_err<'a>(req: &HttpRequest, srv_data: &'a SrvData) -> Result<&'a User, Error> {
    let user = get_user(req, srv_data);
    if user.is_none() {
        return Err(ErrorForbidden(
            "You do not have access to call this resource",
        ));
    }
    Ok(user.unwrap())
}

pub fn is_debug_local(req: &HttpRequest, srv_data: &SrvData) -> bool {
    let info = req.connection_info();
    let host = info.host();
    if !host.starts_with("localhost") {
        return false;
    }
    (*srv_data).head.debug
}

pub fn get_token_user<'a>(req: &HttpRequest, srv_data: &'a SrvData) -> Option<&'a User> {
    let got_token = get_qinpel_token(req);
    if got_token.is_empty() {
        return None;
    }
    let our_tokens = &srv_data.tokens.read().unwrap();
    let found_auth = our_tokens.get(got_token);
    if found_auth.is_none() {
        return None;
    }
    let found_auth = found_auth.unwrap();
    let user_name = &found_auth.user;
    for user in &srv_data.users {
        if user_name == &user.name {
            return Some(user);
        }
    }
    None
}

pub fn get_qinpel_token(req: &HttpRequest) -> &str {
    if let Some(token) = req.headers().get("Qinpel-Token") {
        if let Ok(token) = token.to_str() {
            return token;
        }
    }
    ""
}

pub fn check_app_access(app_name: &str, for_user: &User) -> Result<(), Error> {
    if for_user.master {
        return Ok(());
    } else {
        for user_access in &for_user.access {
            if let Access::APP { name } = user_access {
                if app_name == name {
                    return Ok(());
                }
            }
        }
    }
    Err(ErrorForbidden(liz_debug!(
		"You do not have access to call this resource",
		"check_app_access",
		app_name
	)))
}

pub fn check_dir_access(
    path_ref: &str,
    path_dest: Option<&str>,
    resource: &str,
    for_user: &User,
) -> Result<(), Error> {
    if check_dir_resource(path_ref, &path_dest, resource, for_user) {
        return Ok(());
    } else {
        return Err(ErrorForbidden(liz_debug!(
            "You do not have access to call this resource",
            "check_dir_resource",
            path_ref,
            path_dest,
            resource
        )));
    }
}

fn check_dir_resource(
    path_ref: &str,
    path_dest: &Option<&str>,
    resource: &str,
    for_user: &User,
) -> bool {
    if for_user.master {
        return true;
    }
    if resource == "/dir/list" {
        return check_dir_read(&path_ref, &for_user);
    } else if resource == "/dir/new" {
        return check_dir_write(&path_ref, &for_user);
    } else if resource == "/dir/copy" {
        if let Some(path_dest) = path_dest {
            return check_dir_read(&path_ref, &for_user) && check_dir_write(&path_dest, &for_user);
        }
    } else if resource == "/dir/move" {
        if let Some(path_dest) = path_dest {
            return check_dir_write(&path_ref, &for_user) && check_dir_write(&path_dest, &for_user);
        }
    } else if resource == "/dir/del" {
        return check_dir_write(&path_ref, &for_user);
    } else if resource == "/file/read" {
        return check_dir_read(&path_ref, &for_user);
    } else if resource == "/file/write" {
        return check_dir_write(&path_ref, &for_user);
    } else if resource == "/file/append" {
        return check_dir_write(&path_ref, &for_user);
    } else if resource == "/file/upload" {
        return check_dir_write(&path_ref, &for_user);
    } else if resource == "/file/copy" {
        if let Some(path_dest) = path_dest {
            return check_dir_read(&path_ref, &for_user) && check_dir_write(&path_dest, &for_user);
        }
    } else if resource == "/file/move" {
        if let Some(path_dest) = path_dest {
            return check_dir_write(&path_ref, &for_user) && check_dir_write(&path_dest, &for_user);
        }
    } else if resource == "/file/del" {
        return check_dir_write(&path_ref, &for_user);
    } else {
        eprintln!("[SYSTEM ERROR] We got an unknown resource to check the directory access: {}", resource)
    }
    false
}

pub fn check_dir_read(check_path: &str, for_user: &User) -> bool {
    for user_access in &for_user.access {
        if let Access::DIR { path, can_write: _ } = user_access {
            if check_path.starts_with(path) {
                return true;
            }
        }
    }
    false
}

pub fn check_dir_write(check_path: &str, for_user: &User) -> bool {
    for user_access in &for_user.access {
        if let Access::DIR { path, can_write } = user_access {
            if check_path.starts_with(path) && *can_write {
                return true;
            }
        }
    }
    false
}

pub fn check_cmd_access(cmd_name: &str, for_user: &User) -> Result<(), Error> {
    if for_user.master {
        return Ok(());
    } else {
        for user_access in &for_user.access {
            if let Access::CMD { name, fixed_args: _ } = user_access {
                if cmd_name == name {
                    return Ok(());
                }
            }
        }
    }
    Err(ErrorForbidden(liz_debug!(
		"You do dot have access to call this resource",
		"check_cmd_access",
		cmd_name
	)))
}

pub fn check_sql_access(bas_name: &str, sql_path: &str, for_user: &User) -> Result<(), Error> {
    if for_user.master || bas_name == Base::get_default_bas_name(for_user) {
        return Ok(());
    }
    let mut has_dbs_access = false;
    for user_access in &for_user.access {
        if let Access::BAS { name } = user_access {
            if bas_name == name {
                has_dbs_access = true;
                break;
            }
        }
    }
    if !has_dbs_access {
        return Err(ErrorForbidden(liz_debug!(
            "You do not have access to call this resource",
            "check_sql_access",
            bas_name
        )))
    }
    let mut has_sql_access = false;
    for user_access in &for_user.access {
        if let Access::SQL { path } = user_access {
            if sql_path.starts_with(path)  {
                has_sql_access = true;
                break;
            }
        }
    }
    if !has_sql_access {
        return Err(ErrorForbidden(liz_debug!(
            "You don't have access to call this resource",
            "check_sql_access",
            sql_path
        )))
    }
    Ok(())
}

pub fn check_liz_access(liz_path: &str, for_user: &User) -> Result<(), Error> {
    if for_user.master {
        return Ok(());
    }
    for user_access in &for_user.access {
        if let Access::LIZ { path } = user_access {
            if liz_path.starts_with(path) {
                return Ok(());
            }
        }
    }
    Err(ErrorForbidden(liz_debug!(
        "You do not have access to call this resource",
        "check_liz_access",
        liz_path
    )))
}
