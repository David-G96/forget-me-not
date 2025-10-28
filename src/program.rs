use chrono::{DateTime, Utc};
use colored::Colorize;
use std::str::FromStr;

use crate::{
    cli::Cli,
    config::Config,
    simpledata::{
        self,
        data::{DisplayableSimpleDataVec, SimplePackageData},
        sqlite::{self, try_insert},
    },
};
use rusqlite::{Connection, params};

/// Program of forget-me-not, config and connection are both immutable once initialized
#[derive(Debug)]
pub struct SimpleProgram {
    connection: Connection,
    config: Config,
}

impl SimpleProgram {
    pub fn new(config_str: &str, mut conn: Connection) -> Result<Self, String> {
        let config = Config::from_str(config_str).map_err(|e| e.to_string())?;
        sqlite::try_create_table(&mut conn)?;
        Ok(Self {
            connection: conn,
            config,
        })
    }

    pub fn clear_packages_table(&mut self) -> Result<(), String> {
        const DELETE_ALL: &str = "DELETE FROM Packages";

        self.connection
            .execute(DELETE_ALL, params![])
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    pub fn list_simple_data(&mut self) -> Result<Vec<SimplePackageData>, String> {
        simpledata::sqlite::try_list_all(&mut self.connection)
    }

    pub fn init_db(&mut self) -> Result<(), String> {
        simpledata::sqlite::try_create_table(&mut self.connection)
    }

    pub fn insert_package(&mut self, package: SimplePackageData) -> Result<usize, String> {
        sqlite::try_insert(&mut self.connection, package)
    }

    pub fn run(&mut self, cli: Cli) {
        use crate::cli::CliCommand::*;
        match cli.command() {
            Track => {
                if cli.args().len() < 2 {
                    eprintln!("{}{}",
                    "too less args for command: track\n".red(),
                    "you need at least to provide the package name and source to track a package".blue() );
                    return;
                }
                let name: &str = cli.args().first().unwrap();
                let source: &str = cli.args().get(1).unwrap();
                let description = cli.args().get(2);
                let time_stamp = cli.args().get(3);
                let package = SimplePackageData::new(
                    name.to_string(),
                    source.to_string(),
                    description.map(|s| s.to_string()),
                    time_stamp.map(|s| s.to_string()),
                );

                if let Err(msg) = self.insert_package(package) {
                    eprintln!("failed to insert package. cause: {}", msg);
                }
            }
            List => {
                let result = self.list_simple_data();
                match result {
                    Ok(pkgs) => {
                        println!("{}", DisplayableSimpleDataVec::from(&pkgs));
                    }
                    Err(e) => {
                        eprint!("failed to list packages: {}", e);
                    }
                }
            }
            _ => {
                eprintln!("{}", "unrecognized command".red());
            }
        }
    }
}
