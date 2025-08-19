use std::{env, process::ExitCode, time::Instant};

use chrono::Utc;
use clap::Parser;

use crate::{cli::Cli, models::get_all_package_data, program::Program};

mod cli;
mod config;
mod models;
mod path;
mod program;

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();
    let start = Instant::now();
    let cli = Cli::parse();
    let mut program = Program::init().unwrap();

    match cli.command {
        cli::Commands::Track {
            source: manager,
            package,
            version,
            install_time,
            description,
            args,
        } => {
            let time_stamp = Utc::now();
            let list = program.list_all().unwrap();
            println!("you have tracked {} packages", list.len());
            for pkg in &list {
                println!("{:?}", pkg);
            }

            program
                .track(manager, package, version, time_stamp, description)
                .unwrap();

            let list = program.list_all().unwrap();
            println!("success!\nnow you have tracked {} packages", list.len());
            for pkg in list {
                println!("{:?}", pkg);
            }
        }
        cli::Commands::List {
            manager,
            package,
            args,
        } => {
            let res = get_all_package_data(&mut program.db_conn).unwrap();

            for r in res {
                println!(
                    "{} {} {} {} {{ {} }}",
                    r.package_name,
                    r.source,
                    r.description.unwrap_or("".to_string()),
                    r.version,
                    r.installation
                );
            }
        }
        _ => eprintln!("out of consideration"),
    }

    let end = Instant::now();
    println!("{:2?}", end - start);

    ExitCode::SUCCESS
}
