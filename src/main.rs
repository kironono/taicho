use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

mod config;
mod error;
mod manager;
mod program;
mod task;

use manager::TaskManager;

#[tokio::main]
async fn main() {
    let default_config_filename = format!("{}.yaml", crate_name!());
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .default_value(&default_config_filename),
        )
        .get_matches();

    let config_filename = matches
        .value_of("file")
        .unwrap_or(&default_config_filename)
        .to_string();

    let mng = TaskManager::new(config_filename).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    mng.run().await;
}
