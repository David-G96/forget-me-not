use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use rusqlite::Connection;

const APP_NAME: &str = "ForgetMeNot";
const CONFIG_FILE_NAME: &str = "config.toml";
const DATABASE_FILE_NAME: &str = "package_data.db";

fn get_app_data_local_dir() -> Result<PathBuf, String> {
    if let Some(mut data_dir) = dirs::data_local_dir() {
        data_dir.push(APP_NAME);
        return Ok(data_dir);
    }
    Err("cannot find data local dir!".to_string())
}

fn get_app_config_dir() -> Result<PathBuf, String> {
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push(APP_NAME);
        return Ok(config_dir);
    }
    Err("cannot find config dir!".to_string())
}

fn get_app_db_path() -> Result<PathBuf, String> {
    let mut data_local_dir = get_app_data_local_dir()?;
    data_local_dir.push(DATABASE_FILE_NAME);
    Ok(data_local_dir)
}

fn get_app_config_path() -> Result<PathBuf, String> {
    let mut config_dir = get_app_config_dir()?;
    config_dir.push(CONFIG_FILE_NAME);
    Ok(config_dir)
}

/// create or open the config file.
///
/// will always open in read-only mode
pub fn create_or_open_config_file() -> Result<File, String> {
    let config_path = get_app_config_path()?;

    // 首先检查文件是否存在，如果不存在则创建一个空文件
    if !config_path.exists() {
        let _file = File::create(&config_path).map_err(|e| e.to_string())?;
        // 文件会自动关闭
    }

    // 然后以只读方式打开
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(config_path)
        .map_err(|e| e.to_string())?;

    Ok(file)
}

/// crate or connect to the database
pub fn create_or_connect_database() -> Result<Connection, String> {
    let connection_result = rusqlite::Connection::open(get_app_db_path()?);
    match connection_result {
        Ok(connection) => Ok(connection),
        Err(connection_err) => Err(connection_err.to_string()),
    }
}
