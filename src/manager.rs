use std::{
    process::Stdio,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time,
};

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

    pub async fn run(&self) -> Result<()> {
        let exited_task_count = Arc::new(AtomicUsize::new(0));
        let name_col_length = self
            .programs
            .iter()
            .map(|program| program.name.len())
            .max_by(|x, y| x.cmp(y))
            .unwrap();

        for program in self.programs.clone() {
            let exited_task_count = exited_task_count.clone();

            tokio::task::spawn(async move {
                let name_len = program.name.len();
                let padding = " ".repeat(name_col_length - name_len);
                let tag = format!("{}{}", program.name, padding);

                let mut task: Task = Task::spawn(&program, Stdio::piped(), Stdio::piped())
                    .await
                    .expect(&format!("failed to spawn {} task", tag))
                    .into();

                match task.stdout() {
                    None => {
                        eprintln!("{} | {}", tag, "Unable to read stdout")
                    }
                    Some(stdout) => {
                        let mut reader = BufReader::new(stdout).lines();
                        tokio::task::spawn({
                            let tag = tag.clone();
                            async move {
                                while let Some(line) = reader.next_line().await.unwrap() {
                                    eprintln!("{} | {}", tag, line);
                                }
                            }
                        });
                    }
                }

                match task.stderr() {
                    None => {
                        eprintln!("{} | {}", tag, "Unable to read stderr")
                    }
                    Some(stderr) => {
                        let mut reader = BufReader::new(stderr).lines();
                        tokio::task::spawn({
                            let tag = tag.clone();
                            async move {
                                while let Some(line) = reader.next_line().await.unwrap() {
                                    eprintln!("{} | {}", tag, line);
                                }
                            }
                        });
                    }
                }

                let _exit_result = task.exit_check().await;

                exited_task_count.fetch_add(1, Ordering::Relaxed)
            });
        }

        let programs_len = self.programs.len();
        {
            let exited_task_count = exited_task_count.clone();
            tokio::task::spawn(async move {
                while exited_task_count.load(Ordering::Relaxed) < programs_len {
                    time::sleep(Duration::from_millis(50)).await;
                }
            })
            .await?;
        }

        // return done all task
        Ok(())

        // eprintln!("Ctrl C wait");
        // signal::ctrl_c().await.unwrap();
        //
        // let exit_expire = Instant::now() + Duration::from_secs(10);
        // while exited_task_count.load(Ordering::Relaxed) < self.programs.len() {
        //     if Instant::now() > exit_expire {
        //         eprintln!("Exit waiting timeout.");
        //         break;
        //     }
        //     time::sleep(Duration::from_millis(500)).await;
        // }
        //
        // Ok(())
    }
}
