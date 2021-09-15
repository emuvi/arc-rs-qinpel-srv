use actix_web::error::{Error};
use actix_web::{Responder};

use std::path::{Path, PathBuf};
use std::time::Duration;

static SLEEP_TO_SHUTDOWN: Duration = Duration::from_millis(1000);

pub fn shutdown() -> Result<impl Responder, Error> {
	let result = String::from("QinpelSrv is shutdown...");
	println!("{}", result);
	std::thread::spawn(|| {
		std::thread::sleep(SLEEP_TO_SHUTDOWN);
		std::process::exit(0);
	});
	Ok(result)
}

fn list_folder(folder: PathBuf) -> Result<impl Responder, Error> {
	let mut body = String::from("QinpelSrv ");
    if let Some(folder_name) = folder.file_name() {
        if let Some(folder_name) = folder_name.to_str() {
            body.push_str(folder_name);
        }
    }
	body.push_str(":\n");
	for entry in folder.read_dir()? {
		if let Ok(entry) = entry {
			let path = entry.path();
			if path.is_dir() {
				if let Some(name) = path.file_name() {
					if let Some(name) = name.to_str() {
						body.push_str(name);
						body.push_str("\n");
					}
				}
			}
		}
	}
	Ok(body)
}

pub fn list_app() -> Result<impl Responder, Error> {
    list_folder(Path::new("./run/app").to_owned())
}

pub fn list_cmd() -> Result<impl Responder, Error> {
	list_folder(Path::new("./run/cmd").to_owned())
}

