use actix_web::{web, post};
use serde_derive::Deserialize;

use super::SrvData;

#[derive(Deserialize)]
pub struct Auth {
    name: String,
    pass: String,
}

#[post("/login")]
pub async fn login(auth: web::Form<Auth>, srv_data: SrvData) -> String {
    srv_data.write().unwrap().users.push(super::data::User{
        name: String::from("tst"),
        pass: String::from("tst"),
        master: true,
        access: Vec::new(),
    });
    format!("Welcome {} with {}!", auth.name, auth.pass)
}