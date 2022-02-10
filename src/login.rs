use actix_web::error::ErrorForbidden;
use actix_web::{post, web::Json, HttpResponse};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::data::User;
use crate::SrvData;
use crate::SrvResult;

use std::time::SystemTime;

pub struct Auth {
    pub user: User,
    pub from: SystemTime,
}

#[derive(Deserialize)]
pub struct TryAuth {
    pub name: String,
    pub pass: String,
}


#[derive(Serialize, Deserialize)]
struct Logged {
    pub token: String,
}

#[post("/enter")]
pub async fn enter(auth: Json<TryAuth>, srv_data: SrvData) -> SrvResult {
    let mut user_found: Option<User> = None;
    {
        let users = &srv_data.users;
        for user in users {
            if auth.name == user.name && auth.pass == user.pass {
                user_found = Some(user.clone());
                break;
            }
        }
    }
    if user_found.is_none() {
        return Err(ErrorForbidden("User and pass not found."));
    } else {
        let user = user_found.unwrap();
        let token = generate_token();
        let result = Logged{
            token: token.clone(),
        };
        let auth = Auth {
            user,
            from: std::time::SystemTime::now(),
        };
        {
            srv_data.tokens.write().unwrap().insert(token, auth);
        }
        try_clean_tokens(srv_data);
        return Ok(HttpResponse::Ok().json(result));
    }
}

static CLEAN_INTERVAL: u64 = 24 * 60 * 60;

fn try_clean_tokens(srv_data: SrvData) {
    let elapsed = { srv_data.last_clean.elapsed().unwrap().as_secs() };
    if elapsed > CLEAN_INTERVAL {
        clean_tokens(srv_data);
    }
}

fn clean_tokens(srv_data: SrvData) {
    srv_data.tokens.write().unwrap().retain(|_, auth| {
        let elapsed = auth.from.elapsed().unwrap().as_secs();
        return elapsed < CLEAN_INTERVAL;
    });
}

fn generate_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(36)
        .map(char::from)
        .collect()
}
