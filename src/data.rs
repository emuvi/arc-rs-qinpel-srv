use actix_web::dev::Server;
use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::RwLock;
use std::time::SystemTime;

use crate::pooling::Pooling;
use crate::setup::Head;
use crate::utils;

pub struct Body {
	pub head: Head,
	pub users: Users,
	pub bases: Bases,
	pub pooling: Pooling,
	pub working_dir: String,
	pub tokens: RwLock<HashMap<String, Auth>>,
	pub last_clean: SystemTime,
	pub server: RwLock<Option<Server>>,
}

pub type Users = Vec<User>;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	pub name: String,
	pub pass: String,
	pub home: String,
	pub lang: String,
	pub master: bool,
	pub access: Vec<Access>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Access {
	APP { name: String },
	CMD { name: String, params: Vec<String> },
	DBS { name: String },
	DIR { path: String, write: bool },
}

pub type Bases = Vec<Base>;

#[derive(Serialize, Deserialize)]
pub struct Base {
	pub name: String,
	pub info: String,
}

pub struct Auth {
	pub user: User,
	pub from: SystemTime,
}

#[derive(Deserialize)]
pub struct TryAuth {
	pub name: String,
	pub pass: String,
}

#[derive(Deserialize)]
pub struct RunParams {
	pub params: Vec<String>,
	pub inputs: Vec<String>,
}

#[derive(Deserialize)]
pub struct OnePath {
	pub path: String,
}

#[derive(Deserialize)]
pub struct TwoPath {
	pub origin: String,
	pub destiny: String,
}

#[derive(Deserialize)]
pub struct PathData {
	pub path: String,
	pub base64: bool,
	pub data: String,
}

impl Body {
	pub fn new(head: Head) -> Self {
		let working_dir = Body::init_working_dir();
		let users = Body::init_users(&working_dir);
		let bases = Body::init_bases(&users);
		let pooling = Pooling::new();
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
				home: String::from("./run/dir/root"),
				lang: String::new(),
				master: true,
				access: Vec::new(),
			};
			users.push(user);
		}
		for user in &mut users {
			if user.home.is_empty() {
				user.home = format!("./run/dir/{}", user.name);
			}
			user.home = utils::fix_absolute(working_dir, &user.home);
			fs::create_dir_all(&user.home).expect(&format!(
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
			let default_dbs_name = format!("{}_default_dbs", user.name);
			let has_default_dbs = &bases.iter().any(|base| &base.name == &default_dbs_name);
			if !has_default_dbs {
				let default_dbs_file = "default_dbs.sdb";
				let default_dbs_path = utils::join_paths(&user.home, default_dbs_file);
				let default_dbs_info = format!("sqlite://{}", default_dbs_path);
				let default_dbs = Base {
					name: default_dbs_name,
					info: default_dbs_info,
				};
				bases.push(default_dbs);
			}
		}
		bases
	}
}
