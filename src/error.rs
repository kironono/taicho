use std;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("ConfigFileError: {0}")]
    ConfigFileError(String),
    #[error("ConfigDeserializedError: {0}")]
    ConfigDeserializedError(String),
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("IO error: {0}")]
    IoError(std::io::Error),
    #[error("Process exited with non-zero code: {:#?}. Output: {:#?}", .code, .output)]
    NonZeroExitCode {
        code: Option<i32>,
        output: std::process::Output,
    },
}

impl From<std::io::Error> for TaskError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<std::process::Output> for TaskError {
    fn from(output: std::process::Output) -> Self {
        if output.status.success() {
            panic!("Failed to convert command output to error because the command succeeded. Output: {:#?}", output);
        }
        Self::NonZeroExitCode {
            code: output.status.code(),
            output,
        }
    }
}
