use std::{fs::File, io::Read, path::PathBuf, time::Instant};

use chrono::DateTime;
use rusqlite::Connection;

use crate::{program::SimpleProgram, simpledata::data::SimplePackageData};

mod cli;
mod config;
// mod database;
mod path;
mod program;
mod simpledata;

pub fn main() {
    let start = Instant::now();
    let config_path = PathBuf::from("/Users/davidgao/Desktop/Workspace/forget-me-not/config.toml");
    let conn = Connection::open("/Users/davidgao/Desktop/Workspace/forget-me-not/package_data.db")
        .expect("unable to open sqlite db");

    let mut config = File::open(config_path).expect("unable to open config file");
    let mut buf: Vec<u8> = Vec::new();
    let _ = config
        .read_to_end(&mut buf)
        .expect("unable to read from config");

    let config_str = String::from_utf8(buf).expect("config file includes non-utf8");
    dbg!(&config_str);

    let mut program = SimpleProgram::new(&config_str, conn).expect("failed to parse config file");
    program.clear_packages_table().unwrap();
    dbg!(program.list_simple_data());
    let pkg1 = SimplePackageData {
        id: 0,
        name: "pkg1".to_string(),
        source: "wonderland".into(),
        description: Some("a package from wonderland".to_string()),
        installation: Some(chrono::Utc::now().to_string()),
    };

    program
        .insert_package(pkg1)
        .expect("failed to insert package ");

    dbg!(program.list_simple_data());

    program.clear_packages_table();

    dbg!(program.list_simple_data());

    let end = Instant::now();
    println!("time elapsed: {:.2?}", end - start);
}
