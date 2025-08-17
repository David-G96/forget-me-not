use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use anyhow::{Ok, Result, anyhow, bail};
use dirs::{config_dir, data_dir};
use rusqlite::Connection;

use crate::{cli::Cli, config::PackageManagerConfig, models, path};

#[derive(Debug)]
pub struct Program {
    pub db_conn: Connection,
    pub config: HashMap<String, PackageManagerConfig>,
}

impl Program {
    pub fn init() -> Result<Self> {
        let mut data_path = data_dir().ok_or(anyhow!("cannot find data dir"))?;
        let db_path = {
            data_path.push("forget-me-not");
            if let Err(e) = std::fs::create_dir_all(&data_path) {
                bail!("{}", e);
            }

            data_path.push("data.db");
            data_path
        };
        let db_conn =
            Connection::open(&db_path).map_err(|e| anyhow!("failed to open database: {}", e))?;
        models::init(&db_conn)?;

        let mut config_path = config_dir().ok_or(anyhow!("cannot find config dir"))?;
        config_path.push("config.toml");
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config_path);

        let mut config_file = match config_file {
            Result::Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => File::open(config_path)?,
            _ => {
                bail!("cannot open or create config file")
            }
        };

        let mut buf = String::new();
        config_file
            .read_to_string(&mut buf)
            .map_err(|e| anyhow!("failed to read config file: {}", e))?;
        drop(config_file); // Explicitly close file handle early

        let config: HashMap<String, PackageManagerConfig> =
            toml::from_str(&buf).map_err(|e| anyhow!("failed to parse config: {}", e))?;

        Ok(Self { db_conn, config })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_db_init() {
        let program = Program::init();
        print!("{:#?}", program);
        // let packages: Vec<models::Package> = models::get_all_packages(&program.db_conn).unwrap();

        //for pkg in packages {
        //  println!("{:#?}", pkg);
        //}
    }
}
