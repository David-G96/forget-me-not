use std::process::{Command, ExitCode};

use clap::Parser;
use rusqlite::Connection;
use which::which;

use crate::cli::Cli;

mod cli;
mod db;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let (manager_name, args) = match cli.command {
        cli::Commands::Run { manager, args } => (manager, args),
        cli::Commands::Track { manager, args } => (manager, args),
    };

    if !which::which(&manager_name).is_ok() {
        eprintln!("cannot find `{}` in path!", manager_name);
    }

    

    ExitCode::SUCCESS
}
