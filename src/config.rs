use std::{collections::HashMap, fs::File, io::Write, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "manager")]
    pub package_managers: HashMap<String, PackageManagerConfig>,
}

// 单个包管理器的配置
#[derive(Debug, Deserialize)]
pub struct PackageManagerConfig {
    pub install: InstallConfig,
    pub upgrade: UpgradeConfig,
    pub remove: RemoveConfig,
}

#[derive(Debug, Deserialize)]
pub struct InstallConfig {
    pub template: String,
    #[serde(default)]
    pub pre_args: Option<Vec<String>>,
    #[serde(default)]
    pub post_args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeConfig {
    pub template: String,
    #[serde(default)]
    pub pre_args: Option<Vec<String>>,
    #[serde(default)]
    pub post_args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveConfig {
    pub template: String,
    #[serde(default)]
    pub pre_args: Option<Vec<String>>,
    #[serde(default)]
    pub post_args: Option<Vec<String>>,
}

/// return none if failed
pub fn create_config(path: &Path) -> Option<File> {
    if path.is_dir() {
        let config_file = File::create("config.toml").unwrap();
        return Some(config_file);
    }
    None
}

pub fn init_default_config(file: &mut File) {
    file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
}

pub fn read_config(config_str: &str) -> Config {
    toml::from_str(config_str).unwrap()
}

const DEFAULT_CONFIG: &str = r#"[manager.apt]
install = {template = "install {package}"}
upgrade = { template = "upgrade {package}", pre_args = ["-y"] }
remove = {template = "remove {package}"}

[manager.flatpak]
install = { template = "install {package}", pre_args = [
    "--user",
], post_args = [
    "--assumeyes",
] }
upgrade = {template = "update {package}"}
remove = {template ="remove {package}"}"#;

#[cfg(test)]
mod test {
    use crate::config::Config;

    use std::process::Command;
    #[test]
    fn test() {
        let test_config_input = r#"
[manager.apt]
install = {template = "install {package}"}
upgrade = { template = "upgrade {package}", pre_args = ["-y"] }
remove = {template = "remove {package}"}

[manager.flatpak]
install = { template = "install {package}", pre_args = [
    "--user",
], post_args = [
    "--assumeyes",
] }
upgrade = {template = "update {package}"}
remove = {template ="remove {package}"}"#;

        let config: Config = toml::from_str(&test_config_input).unwrap();

        let pkg_name = "cowsay";
        let pkg_manager = "apt";

        if let Some(pkg_config) = config.package_managers.get(pkg_manager) {
            // 构建命令
            let mut cmd = Command::new("sh");
            cmd.arg("-c");

            // 1. 处理 pre_args
            let mut final_args = Vec::new();
            if let Some(pre_args) = &pkg_config.install.pre_args {
                final_args.extend(pre_args.iter().cloned());
            }

            // 2. 替换 command 模板中的占位符
            let mut full_command_str = pkg_config.install.template.replace("{package}", pkg_name);

            // 3. 处理 post_args
            if let Some(post_args) = &pkg_config.install.post_args {
                full_command_str.push_str(" ");
                full_command_str.push_str(&post_args.join(" "));
            }

            // 构建最终的命令字符串
            final_args.push(full_command_str);

            final_args.insert(0, pkg_manager.to_string());

            let final_args = final_args.join(" ");
            assert_eq!(final_args, "apt install cowsay");
        } else {
            eprintln!("Package manager '{}' not found in config.", pkg_manager);
        }
    }
}
