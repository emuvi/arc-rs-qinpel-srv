use actix_web::error::ErrorForbidden;
use actix_web::{post, web::Json, HttpResponse};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::data::Auth;
use super::data::TryAuth;
use super::data::User;
use super::SrvData;
use super::SrvResult;

static CLEAN_INTERVAL: u64 = 24 * 60 * 60;

#[post("/login")]
pub async fn login(auth: Json<TryAuth>, srv_data: SrvData) -> SrvResult {
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
        let result = format!("{},{}", &token, &user.lang);
        let auth = Auth {
            user,
            from: std::time::SystemTime::now(),
        };
        {
            srv_data.tokens.write().unwrap().insert(token, auth);
        }
        try_clean_tokens(srv_data);
        return Ok(HttpResponse::Ok().body(result));
    }
}

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
