use std::{
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result, anyhow, bail};
pub const APP_DIR_NAME: &str = "forget-me-not";
pub const DB_NAME: &str = "todos.db";

pub fn get_db_path() -> Result<std::path::PathBuf> {
    let mut path = dirs::data_dir().ok_or(anyhow!("cannot locate data dir"))?;
    path.push(APP_DIR_NAME);
    create_dir_all(&path)?;
    path.push(DB_NAME);
    Ok(path)
}

pub fn get_config_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().ok_or(anyhow!("cannot locate data dir"))?;
    path.push(APP_DIR_NAME);
    create_dir_all(&path)?;
    path.push("config.toml");
    Ok(path)
}

pub fn open_or_create_file(path: &Path) -> Result<File> {
    let open_result = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path);

    let file = match open_result {
        Result::Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => File::open(path)?,
        _ => {
            bail!("cannot open or create file: {}", path.display())
        }
    };
    Ok(file)
}
