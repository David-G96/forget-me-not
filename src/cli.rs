use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(name = "forget-me-not", about = "一個跨包管理器工具")]
struct Cli {
    #[arg(short, long, help = "設定日誌等級")]
    log_level: Option<String>,

    #[arg(help = "要使用的包管理器名稱，例如：apt, flatpak, dnf")]
    manager_name: String,

    #[arg(help = "傳遞給包管理器的命令和引數")]
    command_args: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cli = Cli::parse();

        println!("我的日誌等級是：{:?}", cli.log_level);

        // 獲取包管理器名稱
        let manager = &cli.manager_name;

        // 獲取所有要傳遞給包管理器的引數
        let args = &cli.command_args;

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
