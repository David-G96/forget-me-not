use std::str::FromStr;

use crate::{
    config::Config,
    simpledata::{self, data::SimplePackageData, sqlite},
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
}
