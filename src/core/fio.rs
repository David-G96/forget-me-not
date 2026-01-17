use std::path::PathBuf;

use color_eyre::Result as Res;

// pub fn get_config_path() -> Res<PathBuf>{
//     use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, Xdg};

//         // 1. 定义应用策略参数
//         let args = AppStrategyArgs {
//             top_level_domain: "com".to_string(), // 这俩都仅在某些策略中用到，应该不用担心
//             author: "fmn_author".to_string(),
//             app_name: "fmn".to_string(),
//         };

//         // 2. 显式使用 Xdg 策略（确保在 macOS 上使用 ~/.config/fmn）
//         let config_path = Xdg::new(args)
//             .ok()
//             .map(|strategy| strategy.config_dir().join("config.toml"));

//         if let Some(path) = config_path.as_ref().filter(|p| p.exists()) {
//             return Ok(path.to_path_buf());
//         }
    
// }