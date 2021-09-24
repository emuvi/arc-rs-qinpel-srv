use serde::{Deserialize, Serialize};
use serde_json;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2_postgres::{PostgresConnectionManager, postgres::NoTls};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::RwLock;
use std::time::SystemTime;

use super::setup;
use super::utils;

type BaseSQLITE = Pool<SqliteConnectionManager>;
type BasePOSTGRES = Pool<PostgresConnectionManager<NoTls>>;

pub struct Body {
	pub head: setup::Head,
	pub users: Users,
	pub bases: Bases,
	pub tokens: RwLock<HashMap<String, Auth>>,
	pub last_clean: SystemTime,
	pub working: String,
	pub bases_sqlite: HashMap<String, BaseSQLITE>,
	pub bases_postgres: HashMap<String, BasePOSTGRES>,
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
	APP { name: String },
	CMD { name: String, params: Vec<String> },
	DBS { name: String },
	DIR { path: String, write: bool },
}

pub type Bases = Vec<Base>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
	pub name: String,
	pub kind: BaseKind,
	pub info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BaseKind {
	SQLITE,
	POSTGRES,
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
	pub params: Option<Vec<String>>,
	pub inputs: Option<Vec<String>>,
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
	pub fn new(head: setup::Head) -> Self {
		let users_path = Path::new("users.json");
		let mut users = if users_path.exists() {
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
		let working =
			std::env::current_dir().expect("Could not get the current working directory.");
		let working = format!("{}", working.display());
		let mut bases_sqlite: HashMap<String, BaseSQLITE> = HashMap::new();
		let mut bases_postgres: HashMap<String, BasePOSTGRES> = HashMap::new();
		for base in &bases {
			match base.kind {
				BaseKind::SQLITE => {
					let manager = SqliteConnectionManager::file(&base.info);
    				let pool = Pool::new(manager).unwrap();
					bases_sqlite.insert(base.name.clone(), pool);
				},
				BaseKind::POSTGRES => {
					let manager = PostgresConnectionManager::new(
						base.info.parse().unwrap(),
						NoTls,
					);
					let pool = r2d2::Pool::new(manager).unwrap();
					bases_postgres.insert(base.name.clone(), pool);
				},
			}
		}
		Body::init_users(&mut users, &working);
		Body {
			head,
			users,
			bases,
			tokens: RwLock::new(HashMap::new()),
			last_clean: SystemTime::now(),
			working,
			bases_sqlite,
			bases_postgres,
		}
	}

	fn init_users(users: &mut Users, working: &str) {
		let has_root = users.into_iter().any(|user| user.name == "root");
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
		for user in users {
			if user.home.is_empty() {
				user.home = format!("./run/dir/{}", user.name);
			}
			user.home = utils::fix_absolute(working, &user.home);
			fs::create_dir_all(&user.home).expect(&format!(
				"Could not create the {} home dir on: {}",
				user.name, user.home
			));
		}
	}
}
