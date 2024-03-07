use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    env_files_path: String,
    gpg_key_id: String,
}

impl AppConfig {
    // Getter for env_files_path
    pub fn get_env_files_path(&self) -> &str {
        &self.env_files_path
    }

    // Getter for gpg_key_id
    pub fn get_gpg_key_id(&self) -> &str {
        &self.gpg_key_id
    }

    // Reads the configuration from a given file path and initializes AppConfig
    pub fn new(config_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        match serde_json::from_str(&contents) {
            Ok(cfg) => Ok(cfg),
            Err(e) if e.is_eof() => Err("The configuration file is empty or not properly formatted.".into()),
            Err(e) => Err(e.into()),
        }
    }
}

