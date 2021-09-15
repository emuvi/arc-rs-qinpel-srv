use serde::{Deserialize, Serialize};
use serde_json;

use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use super::setup;

pub struct Body {
	pub head: setup::Head,
	pub users: Users,
	pub bases: Bases,
	pub tokens: HashMap<String, User>,
}

pub type Users = Vec<User>;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
	pub name: String,
	pub pass: String,
	pub master: bool,
	pub access: Vec<Access>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Access {
	APP {name: String},
	CMD {name: String, params: Vec<String>},
	DBS {name: String},
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
	SQLITE, POSTGRES,
}

impl Body {
	pub fn new(head: setup::Head) -> Self {
		let users_path = Path::new("users.json");
		let users = if users_path.exists() {
			serde_json::from_reader(File::open(users_path).expect("Could not open the users file."))
				.expect("Could not parse the users file.")
		} else {
			Users::new()
		};
		let bases_path = Path::new("bases.json");
		let bases = if bases_path.exists() {
			serde_json::from_reader(File::open(bases_path).expect("Could not open the bases file."))
				.expect("Could not parse the bases file.")
		} else {
			Bases::new()
		};
		Body { head, users, bases, tokens: HashMap::new() }
	}
}