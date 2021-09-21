use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

use super::setup;

pub struct Body {
	pub head: setup::Head,
	pub users: Users,
	pub bases: Bases,
	pub tokens: HashMap<String, Auth>,
	pub last_clean: SystemTime,
}

pub struct Auth {
	pub user: User,
	pub from: SystemTime,
}

pub type Users = Vec<User>;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	pub name: String,
	pub pass: String,
	pub lang: String,
	pub master: bool,
	pub access: Vec<Access>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Access {
	APP { name: String },
	CMD { name: String, params: Vec<String> },
	DBS { name: String },
}

impl std::fmt::Display for Access {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Access::APP{name} => write!(f, "/run/app/{}/", name),
			Access::CMD{name, params: _} => write!(f, "/run/cmd/{}/", name),
			Access::DBS{name} => write!(f, "/run/dbs/{}/", name),
		}
    }
}

pub type Bases = Vec<Base>;

#[derive(Serialize, Deserialize)]
pub struct Base {
	pub name: String,
	pub kind: BaseKind,
	pub info: String,
}

#[derive(Serialize, Deserialize)]
pub enum BaseKind {
	SQLITE,
	POSTGRES,
}

impl Body {
	pub fn new(head: setup::Head) -> Self {
		let users_path = Path::new("users.json");
		let mut users = if users_path.exists() {
			serde_json::from_reader(File::open(users_path).expect("Could not open the users file."))
				.expect("Could not parse the users file.")
		} else {
			Users::new()
		};
		Body::init_users(&mut users);
		let bases_path = Path::new("bases.json");
		let bases = if bases_path.exists() {
			serde_json::from_reader(File::open(bases_path).expect("Could not open the bases file."))
				.expect("Could not parse the bases file.")
		} else {
			Bases::new()
		};
		Body {
			head,
			users,
			bases,
			tokens: HashMap::new(),
			last_clean: SystemTime::now(),
		}
	}

	fn init_users(users: &mut Users) {
		let has_root = users.into_iter().any(|user| user.name == "root");
		if !has_root {
			let root = User {
				name: String::from("root"),
				pass: String::new(),
				lang: String::new(),
				master: true,
				access: Vec::new(),
			};
			users.push(root);
		}
	}
}
