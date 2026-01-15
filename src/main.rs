// use std::{fs::File, io::Read, path::PathBuf, time::Instant};

// use rusqlite::Connection;

// use crate::{cli::Cli, app::SimpleApp};

// mod cli;
// mod config;
// mod fs;
// mod app;
// mod simpledata;

// pub fn main() {
//     let start = Instant::now();
//     let cli = Cli::parse();

//     let config_path = PathBuf::from("./config.toml");
//     let conn = Connection::open("./package_data.db").expect("unable to open sqlite db");

//     let mut config = File::open(config_path).expect("unable to open config file");
//     let mut buf: Vec<u8> = Vec::new();
//     let _ = config
//         .read_to_end(&mut buf)
//         .expect("unable to read from config");

//     let config_str = String::from_utf8(buf).expect("config file includes non-utf8");

//     let mut program = SimpleApp::new(&config_str, conn).expect("failed to parse config file");

//     program.run(cli);

//     let end = Instant::now();
//     println!("time elapsed: {:.2?}", end - start);
// }

use clap::Parser;

use crate::core::cli::Cli;

mod core;

fn main() {
    let cli = Cli::parse();
}
