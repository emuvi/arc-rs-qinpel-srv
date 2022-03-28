use serde::{Deserialize, Serialize};

use std::time::SystemTime;

#[derive(Debug)]
pub struct Authed {
    pub user: String,
    pub from: SystemTime,
}

pub type Users = Vec<User>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub home: String,
    pub lang: String,
    pub master: bool,
    pub access: Vec<Access>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Access {
    APP {
        name: String,
    },
    DIR {
        path: String,
        can_write: bool,
    },
    CMD {
        name: String,
        args: Option<Vec<String>>,
    },
    BAS {
        name: String,
    },
    REG {
        name: String,
    },
    SQL {
        path: String,
    },
    LIZ {
        path: String,
    },
}