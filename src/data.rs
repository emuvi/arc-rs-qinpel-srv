use actix_web::dev::Server;
use liz::liz_paths;
use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::login::Auth;
use crate::pooling::Pool;
use crate::setup::Head;

pub struct Body {
    pub head: Head,
    pub users: Users,
    pub bases: Bases,
    pub pooling: Pool,
    pub working_dir: String,
    pub tokens: RwLock<HashMap<String, Auth>>,
    pub last_clean: SystemTime,
    pub server: RwLock<Option<Server>>,
}

pub type Users = Vec<User>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub home: String,
    pub master: bool,
    pub access: Vec<Access>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Access {
    APP { name: String },
    CMD { name: String, params: Vec<String> },
    DIR { path: String, write: bool },
    SQL { name: String },
    LIZ { file: bool, eval: bool },
}

pub type Bases = Vec<Base>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    pub name: String,
    pub info: String,
}

impl Body {
    pub fn new(head: Head) -> Self {
        let working_dir = Body::init_working_dir();
        let users = Body::init_users(&working_dir);
        let bases = Body::init_bases(&users);
        let pooling = Pool::new();
        Body {
            head,
            users,
            bases,
            pooling,
            working_dir,
            tokens: RwLock::new(HashMap::new()),
            last_clean: SystemTime::now(),
            server: RwLock::new(None),
        }
    }

    fn init_working_dir() -> String {
        let current_dir =
            std::env::current_dir().expect("Could not get the current working directory.");
        format!("{}", current_dir.display())
    }

    fn init_users(working_dir: &str) -> Users {
        let users_path = Path::new("users.json");
        let mut users = if users_path.exists() {
            serde_json::from_reader(File::open(users_path).expect("Could not open the users file."))
                .expect("Could not parse the users file.")
        } else {
            Users::new()
        };
        let has_root = &users.iter().any(|user| user.name == "root");
        if !has_root {
            let user = User {
                name: String::from("root"),
                pass: String::new(),
                home: String::from("./dir/root"),
                master: true,
                access: Vec::new(),
            };
            users.push(user);
        }
        for user in &mut users {
            if user.home.is_empty() {
                user.home = format!("./dir/{}", user.name);
            }
            user.home = liz_paths::path_join_if_relative(working_dir, &user.home).expect(&format!(
                "Could not join the working dir {} with the home {}",
                working_dir, user.home
            ));
            std::fs::create_dir_all(&user.home).expect(&format!(
                "Could not create the {} home dir on: {}",
                user.name, user.home
            ));
        }
        users
    }

    fn init_bases(users: &Users) -> Bases {
        let bases_path = Path::new("bases.json");
        let mut bases = if bases_path.exists() {
            serde_json::from_reader(File::open(bases_path).expect("Could not open the users file."))
                .expect("Could not parse the users file.")
        } else {
            Bases::new()
        };
        for user in users {
            let default_sql_name = format!("{}_default_sql", user.name);
            let has_default_sql = &bases.iter().any(|base| &base.name == &default_sql_name);
            if !has_default_sql {
                let default_sql_file = "default_sql.sdb";
                let default_sql_path =
                    liz_paths::path_join(&user.home, default_sql_file).expect(&format!(
                        "Could not join the user home {} with default sql {}",
                        &user.home, default_sql_file
                    ));
                let default_sql_info = format!("sqlite://{}", default_sql_path);
                let default_sql = Base {
                    name: default_sql_name,
                    info: default_sql_info,
                };
                bases.push(default_sql);
            }
        }
        bases
    }
}
