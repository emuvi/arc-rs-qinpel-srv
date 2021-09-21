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
		let mut has_root = false;
		for user in users {
			if user.name == "root" {
				has_root = true;
			}
			if user.home.is_empty() {
				user.home = format!("./run/dir/{}", user.name);
			}
			std::fs::create_dir_all(user.home).expect(&format!(
				"Could not create the {} home dir on: {}",
				user.name, user.home
			));
		}
		if !has_root {
			let user = User {
				name: String::from("root"),
				pass: String::new(),
				home: String::from("./run/dir/root"),
				lang: String::new(),
				master: true,
				access: Vec::new(),
			};
			std::fs::create_dir_all(user.home).expect(&format!(
				"Could not create the {} home dir on: {}",
				user.name, user.home
			));
			users.push(user);
		}
	}
}
