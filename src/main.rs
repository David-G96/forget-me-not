use std::process::ExitCode;

use clap::Parser;
use rusqlite::Connection;

use crate::{cli::Cli, path::get_db_path};

mod cli;
mod config;
mod path;
mod program;
mod models;

fn main() -> ExitCode {
    let cli = Cli::parse();

    ExitCode::SUCCESS
}
