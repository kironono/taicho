use std::{
    io,
    process::{Output, Stdio},
};

use anyhow::Result;
use tokio::{
    process::{Child, ChildStderr, ChildStdout, Command},
    signal,
};

use crate::{error::AppError, program::Program};

enum ExitReason {
    CtrlC,
    TaskFinished(io::Result<Output>),
}

pub enum ExitResult {
    Output(Output),
    Interrupted,
}

pub struct Task {
    child: Child,
}

impl Task {
    pub async fn spawn(program: &Program, stdout: Stdio, stderr: Stdio) -> Result<Self> {
        let child = Command::new(&"/bin/sh")
            .args(vec!["-c", &program.command])
            .stdout(stdout)
            .stderr(stderr)
            .spawn()?;

        Ok(Self { child })
    }

    pub fn stdout(&mut self) -> Option<ChildStdout> {
        self.child.stdout.take()
    }

    pub fn stderr(&mut self) -> Option<ChildStderr> {
        self.child.stderr.take()
    }

    pub async fn exit_check(self) -> Result<ExitResult, AppError> {
        let exit_reason = {
            tokio::select! {
                r = tokio::task::spawn(async move {
                    self.child.wait_with_output().await
                }) => ExitReason::TaskFinished(
                    r.unwrap_or_else(|err| Err(io::Error::new(io::ErrorKind::Other, err)))
                ),
                _ = signal::ctrl_c() => ExitReason::CtrlC,
            }
        };

        match exit_reason {
            ExitReason::TaskFinished(result) => match result {
                Ok(output) => {
                    if output.status.success() {
                        Ok(ExitResult::Output(output))
                    } else {
                        Err(AppError::TaskExitError("".to_string()))
                    }
                }
                Err(_e) => Err(AppError::TaskExitError("".to_string())),
            },
            ExitReason::CtrlC => Ok(ExitResult::Interrupted),
        }
    }
}
