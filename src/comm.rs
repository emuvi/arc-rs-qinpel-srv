use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TryAuth {
    pub name: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize)]
pub struct Logged {
    pub lang: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ArgsInputs {
    pub args: Option<Vec<String>>,
    pub inputs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PathParams {
    pub path: String,
    pub params: Option<Vec<String>>,
}
