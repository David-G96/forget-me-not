use std::process::ExitCode;

use clap::Parser;
use rusqlite::Connection;

use crate::{cli::Cli, db::get_db_path};

mod cli;
mod config;
mod db;
mod fmn;
mod models;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let path = db::get_db_path().unwrap();
    let mut conn = Connection::open(path).unwrap();
    db::create_table(&mut conn).unwrap();

    match cli.command {
        cli::Commands::Run { manager, args } => {}
        cli::Commands::Track { manager, args } => {
            if which::which(&manager).is_err() {
                eprintln!("cannot find `{}` in path!", manager);
                return ExitCode::FAILURE;
            }
            let path = get_db_path().unwrap();
            let mut conn = Connection::open(path).unwrap();
            db::add_package(
                &mut conn,
                manager.clone(),
                args[0].clone(),
                args[1].clone(),
                args[2].clone(),
            )
            .unwrap();

            println!("tracked package: {} in manager: {}", args[0], manager);
        }
        cli::Commands::List { manager, args } => {
            let path = db::get_db_path().unwrap();
            let mut conn = Connection::open(path).unwrap();
            let res = db::query_all(&mut conn);

            for r in res {
                println!("{:?}", r);
            }
        }
    };

    ExitCode::SUCCESS
}
