use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgramConfig {
    pub name: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub programs: Vec<ProgramConfig>,
}

impl Config {
    pub fn from_file(path: String) -> Result<Self, AppError> {
        let buf = PathBuf::from(path.to_string());
        let config_str = match fs::read_to_string(buf) {
            Ok(config_str) => config_str,
            Err(_) => return Err(AppError::ConfigFileError("file not found".to_string())),
        };
        match serde_yaml::from_str(&config_str) {
            Ok(config) => Ok(config),
            Err(_) => Err(AppError::ConfigDeserializedError(
                "invalid config".to_string(),
            )),
        }
    }
}
