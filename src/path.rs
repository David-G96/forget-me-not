use std::{
    fs::{File, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result, anyhow, bail};
pub const APP_DIR_NAME: &str = "forget-me-not";
pub const DB_NAME: &str = "data.db";

/// get the supposed db file path on this computer
pub fn get_db_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().ok_or(anyhow!("Cannot locate system data directory"))?;
    path.push(APP_DIR_NAME);
    create_dir_all(&path)
        .with_context(|| format!("Failed to create app directory at {:?}", path))?;
    path.push(DB_NAME);
    Ok(path)
}

/// get the supposed config file path on this computer,
/// will not create the config file
pub fn get_config_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().ok_or(anyhow!("Cannot locate system data directory"))?;
    path.push(APP_DIR_NAME);
    create_dir_all(&path)
        .with_context(|| format!("Failed to create app directory at {:?}", path))?;
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
            bail!("Cannot open or create file: {}", path.display())
        }
    };
    Ok(file)
}
