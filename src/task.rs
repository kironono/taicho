use std::{io::Result, process::Stdio};

use tokio::process::{Child, Command};

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
}
