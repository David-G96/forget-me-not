use std::{fs::File, io::Read, path::PathBuf, time::Instant};

use chrono::{DateTime, Utc};
use rusqlite::Connection;

use crate::{
    cli::Cli,
    program::SimpleProgram,
    simpledata::data::{DisplayableSimpleDataVec, SimplePackageData},
};

mod cli;
mod config;
// mod database;
mod path;
mod program;
mod simpledata;

pub fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    let config_path = PathBuf::from("/Users/davidgao/Desktop/Workspace/forget-me-not/config.toml");
    let conn = Connection::open("/Users/davidgao/Desktop/Workspace/forget-me-not/package_data.db")
        .expect("unable to open sqlite db");

    let mut config = File::open(config_path).expect("unable to open config file");
    let mut buf: Vec<u8> = Vec::new();
    let _ = config
        .read_to_end(&mut buf)
        .expect("unable to read from config");

    let config_str = String::from_utf8(buf).expect("config file includes non-utf8");

    let mut program = SimpleProgram::new(&config_str, conn).expect("failed to parse config file");

    program.run(cli);

    let end = Instant::now();
    println!("time elapsed: {:.2?}", end - start);
}
