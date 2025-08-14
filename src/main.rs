use std::process::{Command, ExitCode};

use rusqlite::{Connection, Result};
use which::which;

mod cli;
mod db;
mod todo;

fn main() -> ExitCode {
    let db_path = match get_db_path() {
        Some(path) => path,
        None => {
            eprintln!("無法確定或創建資料庫目錄。");
            return ExitCode::FAILURE;
        }
    };

    let conn = match Connection::open(&db_path) {
        Ok(conn) => {
            println!("success");
            conn
        }
        Err(e) => {
            eprintln!("failed: {:?}", e);
            return ExitCode::FAILURE;
        }
    };

    let cmd_to_check = "winget";
    match which(cmd_to_check) {
        Ok(path) => {
            println!("found {}, path {}", cmd_to_check, path.display());

            let mut cmd = Command::new(&path);
            cmd.arg("update");

            match cmd.output() {
                Ok(output) => {
                    if output.status.success() {
                        print!("命令行成功！");
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("输出:\n{}", stdout);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("失败!\n{}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        Err(_) => {
            eprintln!("{} not found", cmd_to_check);
        }
    }

    ExitCode::SUCCESS
}

fn get_db_path() -> Option<std::path::PathBuf> {
    // 1. 獲取使用者資料目錄
    if let Some(mut path) = dirs::data_dir() {
        // 2. 為你的應用程式創建一個子目錄
        path.push("forget-me-not");
        // 3. 確保這個子目錄存在
        if let Err(_) = std::fs::create_dir_all(&path) {
            // 如果無法創建，就返回 None
            return None;
        }

        // 4. 在子目錄中建立資料庫檔案的完整路徑
        path.push("todos.db");
        Some(path)
    } else {
        None
    }
}
