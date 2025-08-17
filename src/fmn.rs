use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use anyhow::{Ok, Result, anyhow};
use dirs::{config_dir, data_dir};
use rusqlite::Connection;

use crate::{cli::Cli, config::PackageManagerConfig, db};

#[derive(Debug)]
pub struct FMN {
    pub db_path: PathBuf,
    pub db_conn: Connection,
    pub config_path: PathBuf,
    pub config: HashMap<String, PackageManagerConfig>,
}

impl FMN {
    pub fn init() -> Result<Self> {
        let db_path = data_dir().ok_or(anyhow!("cannot find data dir"))?;
        let db_conn =
            Connection::open(&db_path).map_err(|e| anyhow!("failed to open database: {}", e))?;
        
       
        let config_path = config_dir().ok_or(anyhow!("cannot find config dir"))?;
        let mut config_file =
            File::open(&config_path).map_err(|e| anyhow!("failed to open config file: {}", e))?;

        let mut buf = String::new();
        config_file
            .read_to_string(&mut buf)
            .map_err(|e| anyhow!("failed to read config file: {}", e))?;
        drop(config_file); // Explicitly close file handle early

        let config: HashMap<String, PackageManagerConfig> =
            toml::from_str(&buf).map_err(|e| anyhow!("failed to parse config: {}", e))?;

        Ok(Self {
            db_path,
            db_conn,
            config_path,
            config,
        })
    }

    pub fn track(
        &mut self,
        manager: String,
        package: String,
        version: String,
        install_time: String,
    ) -> Result<()> {

        Ok(())
    }
}
