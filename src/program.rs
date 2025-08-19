use std::{collections::HashMap, io::Read};

use anyhow::{Ok, Result, anyhow};
use chrono::Utc;
use log::info;
use rusqlite::Connection;

use crate::{
    config::PackageManagerConfig,
    models::{self, Package},
    path,
};

#[derive(Debug)]
pub struct Program {
    pub db_conn: Connection,
    pub config: HashMap<String, PackageManagerConfig>,
}

impl Program {
    pub fn init() -> Result<Self> {
        let db_path = path::get_db_path()?;
        let db_conn =
            Connection::open(&db_path).map_err(|e| anyhow!("failed to open database: {}", e))?;
        models::init_db(&db_conn)?;

        let config_path = path::get_config_path()?;
        let mut config_file = path::open_or_create_file(&config_path)?;

        let mut buf = String::new();
        config_file
            .read_to_string(&mut buf)
            .map_err(|e| anyhow!("failed to read config file: {}", e))?;
        drop(config_file); // Explicitly close file handle early

        // 会panic
        let config: HashMap<String, PackageManagerConfig> =
            toml::from_str(&buf).map_err(|e| anyhow!("failed to parse config: {}", e))?;

        Ok(Self { db_conn, config })
    }

    pub fn track(
        &mut self,
        manager: String,
        package: String,
        version: String,
        time_stamp: chrono::DateTime<Utc>,
        description: Option<String>,
    ) -> Result<()> {
        let id = models::insert_package(
            &mut self.db_conn,
            &package,
            &manager,
            description.as_deref(),
        )?;
        let version_id = models::insert_package_version(&mut self.db_conn, id, &version)?;
        models::insert_installation(&mut self.db_conn, version_id, "install", time_stamp)?;
        Ok(())
    }

    pub fn list_all(&self) -> anyhow::Result<Vec<Package>> {
        models::get_all_packages(&self.db_conn)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_db_init() {
        let program = Program::init();
        let packages: Vec<models::Package> =
            models::get_all_packages(&program.unwrap().db_conn).unwrap();

        for pkg in packages {
            println!("{:#?}", pkg);
        }
    }
}
