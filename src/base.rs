use liz::{liz_dbg_errs, liz_paths};
use serde::{Deserialize, Serialize};

use crate::auth::User;

pub type Bases = Vec<Base>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    pub name: String,
    pub link: String,
}

impl Base {
    pub fn get_default_base_name(for_user: &User) -> String {
        format!("{}_default_dbs", for_user.name)
    }

    pub fn get_default_base_link(for_user: &User) -> String {
        let default_dbs_file = "default_dbs.sdb";
        let default_dbs_path =
            liz_paths::path_join(&for_user.home, default_dbs_file).expect(&liz_dbg_errs!(
                "Could not join the user home with default sql",
                &for_user.home,
                &default_dbs_file
            ));
        format!("sqlite://{}", default_dbs_path)
    }
}
