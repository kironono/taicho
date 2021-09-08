use std::process::Stdio;

use anyhow::Result;
use tokio::process::{Child, ChildStderr, ChildStdout, Command};

use crate::program::Program;

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

    pub async fn exit_check(self) -> Result<()> {
        let _result = self.child.wait_with_output().await;
        Ok(())
    }
}
