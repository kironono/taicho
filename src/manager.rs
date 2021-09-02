use crate::{config::Config, error::AppError};

#[derive(Debug, Clone)]
pub struct TaskManager {
    pub config: Config,
}

impl TaskManager {
    pub fn new(config_filename: String) -> Result<Self, AppError> {
        let config = match Config::from_file(config_filename) {
            Ok(config) => config,
            Err(err) => return Err(err),
        };
        Ok(Self { config })
    }

    pub fn run(&self) {
        println!("Hello, world!");
    }
}
