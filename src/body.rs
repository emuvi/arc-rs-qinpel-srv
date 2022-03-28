use actix_web::dev::Server;
use liz::{liz_dbg_errs, liz_paths};
use serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::auth::{Authed, User, Users};
use crate::base::{Base, Bases};
use crate::pooling::Pool;
use crate::setup::Head;

#[derive(Debug)]
pub struct Body {
    pub head: Head,
    pub users: Users,
    pub bases: Bases,
    pub pooling: Pool,
    pub srv_dir: String,
    pub server: RwLock<Option<Server>>,
    pub tokens: RwLock<HashMap<String, Authed>>,
    pub last_clean: SystemTime,
}

impl Body {
    pub fn new(head: Head) -> Self {
        let srv_dir = Body::init_working_dir();
        let users = Body::init_users(&srv_dir);
        let bases = Body::init_bases(&users);
        let pooling = Pool::new();
        Body {
            head,
            users,
            bases,
            pooling,
            srv_dir,
            server: RwLock::new(None),
            tokens: RwLock::new(HashMap::new()),
            last_clean: SystemTime::now(),
        }
    }

    fn init_working_dir() -> String {
        let current_dir =
            std::env::current_dir().expect("Could not get the current working directory");
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
                lang: String::new(),
                master: true,
                access: Vec::new(),
            };
            users.push(user);
        }
        for user in &mut users {
            if user.home.is_empty() {
                user.home = format!("./dir/{}", user.name);
            }
            user.home =
                liz_paths::path_join_if_relative(working_dir, &user.home).expect(&liz_dbg_errs!(
                    "Could not join the working dir with the home",
                    working_dir,
                    user.home
                ));
            std::fs::create_dir_all(&user.home)
                .expect(&liz_dbg_errs!("Could not create the home dir", user.home));
        }
        users
    }

    fn init_bases(users: &Users) -> Bases {
        let bases_path = Path::new("bases.json");
        let mut bases = if bases_path.exists() {
            serde_json::from_reader(File::open(bases_path).expect("Could not open the bases file"))
                .expect("Could not parse the bases file")
        } else {
            Bases::new()
        };
        for user in users {
            let default_base_name = Base::get_default_base_name(user);
            let has_default_dbs = &bases.iter().any(|base| &base.name == &default_base_name);
            if !has_default_dbs {
                let default_dbs = Base {
                    name: default_base_name,
                    link: Base::get_default_base_link(user),
                };
                bases.push(default_dbs);
            }
        }
        bases
    }
}
