use clap::Parser;
use log::info;

use crate::_models::Package;

#[derive(Debug, Parser)]
#[command(
    name = "forget-me-not",
    about = "a cross-platform/package manager tracker"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[non_exhaustive]
#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// run command and track
    Run {
        /// source name
        manager: String,
        /// args
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// track only
    Track {
        /// source name
        source: String,
        /// package name
        package: String,
        /// sematic version
        version: String,
        /// install time in UTC, current time as default
        install_time: Option<String>,
        /// simple description, none as default
        description: Option<String>,
        /// extra args
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// list all tracked package
    List {
        /// package manager
        manager: Option<String>,
        /// package name
        package: Option<String>,
        /// extra args
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

pub fn print_packages<'a, T>(packages: T)
where
    T: Iterator<Item = &'a Package>,
{
    info!("printing packages");
    for pkg in packages {
        println!("{:#?}", pkg);
    }
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::vec;

    #[test]
    fn test() {
        // 獲取包管理器名稱
        let manager = "apt";

        // 獲取所有要傳遞給包管理器的引數
        let args = vec!["list"];

        // 檢查包管理器是否存在
        // 這是為了保險，建議先用 which::which(manager) 檢查一下
        if let Err(_) = which::which(manager) {
            eprintln!("錯誤：包管理器 '{}' 不存在於 PATH 中。", manager);
            return;
        }

        // 創建並執行命令
        println!("正在調用 '{}'，參數是：{:?}", manager, args);
        let output = Command::new(manager)
            .args(args)
            .output()
            .expect("無法執行命令");
        if output.status.success() {
            println!("命令成功！\n{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("命令失敗！\n{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
