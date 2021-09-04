use std::process::Stdio;

use tokio::signal;

use crate::{config::Config, error::AppError, program::Program, task::Task};

#[derive(Debug, Clone)]
pub struct TaskManager {
    pub config: Config,
    pub programs: Vec<Program>,
}

impl TaskManager {
    pub fn new(config_filename: String) -> Result<Self, AppError> {
        let config = match Config::from_file(config_filename) {
            Ok(config) => config,
            Err(err) => return Err(err),
        };

        let programs = config
            .programs
            .iter()
            .cloned()
            .map(|pc| Program {
                name: pc.name,
                command: pc.command,
            })
            .collect();

        Ok(Self { config, programs })
    }

    pub async fn run(&self) {
        // let name_col_length = programs
        //     .iter()
        //     .map(|program| program.name.len())
        //     .max_by(|x, y| x.cmp(y))
        //     .unwrap();

        for program in self.programs.clone() {
            tokio::task::spawn(async move {
                let tag = &program.name;

                let _task: Task = Task::spawn(&program, Stdio::inherit(), Stdio::inherit())
                    .await
                    .expect(&format!("failed to spawn {} task", tag))
                    .into();
            });
        }

        signal::ctrl_c().await.unwrap();
    }
}
