use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("ConfigFileError: {0}")]
    ConfigFileError(String),
    #[error("ConfigDeserializedError: {0}")]
    ConfigDeserializedError(String),
    #[error("TaskExitError: {0}")]
    TaskExitError(String),
}
