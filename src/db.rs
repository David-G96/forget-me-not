use std::path::PathBuf;

use rusqlite::Connection;

const APP_DIR_NAME: &str = "forget-me-not";
const DB_NAME: &str = "todos.db";

fn get_db_path() -> Option<std::path::PathBuf> {
    // 1. 獲取使用者資料目錄
    if let Some(mut path) = dirs::data_dir() {
        // 2. 為你的應用程式創建一個子目錄
        path.push(APP_DIR_NAME);
        // 3. 確保這個子目錄存在
        if let Err(_) = std::fs::create_dir_all(&path) {
            // 如果無法創建，就返回 None
            return None;
        }

        // 4. 在子目錄中建立資料庫檔案的完整路徑
        path.push(DB_NAME);
        Some(path)
    } else {
        None
    }
}
