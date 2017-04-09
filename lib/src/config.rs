use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde_json;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Parse(serde_json::error::Error),
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Project {
    pub id: String,
    pub repo: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SlackConfig {
    pub user: String,
    pub token: String,
    pub channel: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct StashConfig {
    pub base_url: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AwsConfig {
    pub access_key: String,
    pub secret: String,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config {
    pub db: String,
    pub projects: Vec<Project>,
    pub users: HashMap<String, String>,
    pub slack: SlackConfig,
    pub stash: StashConfig,
    pub aws: AwsConfig,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let mut f = try!(File::open(&path).map_err(ConfigError::Io));
        let mut contents = String::new();
        try!(f.read_to_string(&mut contents).map_err(ConfigError::Io));
        Ok(try!(serde_json::from_str(&contents).map_err(ConfigError::Parse)))
    }
}
