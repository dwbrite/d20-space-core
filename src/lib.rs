pub mod players;
pub mod auth; // Should the even be pub?
pub mod maps;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    appdata: String,
}

#[derive(Debug, Deserialize)]
pub struct Local {
    config: Config
}

#[derive(Debug, Deserialize)]
pub struct Server {
    local: Local,
}

impl Server {

}
