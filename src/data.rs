use std::fs;
use std::path::Path;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use super::setup;

pub struct Body {
    pub setup_head: setup::Head,
    pub master_token: String,
}

impl Body {
    pub fn new(head: setup::Head) -> Self {
        Body{
            setup_head: head,
            master_token: read_master_token()
        }
    }
}

fn read_master_token() -> String {
	let path = Path::new("master-token.txt");
	if path.exists() {
		return fs::read_to_string("master-token.txt").expect("Error: Could not read the token file.");
	} else {
		let new_token: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(27)
			.map(char::from)
			.collect();
		fs::write(path, &new_token).expect("Error: Could not write the token file.");
		return new_token;
	}
}