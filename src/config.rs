use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

// 对应于 serde = { version = "1.0.219", features = ["derive"] } 这样的内联表
#[derive(Debug, Deserialize, PartialEq)]
pub struct TemplateCommand {
    pub template: String,
    //#[serde(default)] // features 可能是可选的
    //pub features: Vec<String>,
    // ... 其他可能的字段，如 registry, path 等
}

// 使用 #[serde(untagged)] 来告诉 Serde 尝试按顺序匹配每一个变体，
// 直到成功为止，而不需要一个额外的标签字段来区分它们。
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Command {
    // Serde 会首先尝试将数据解析为 Inline 变体（一个表/结构体）
    Inline(TemplateCommand),
    // 如果失败，它会尝试将其解析为 Simple 变体（一个字符串）
    Simple(String),
}

/// config for package managers, e.g. apt, dnf
/// this should contains commands for install, upgrade, remove .etc
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct PMConfig {
    install: Command,
    upgrade: Command,
    remove: Command,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Config {
    manager: HashMap<String, PMConfig>,
}

impl Config {
    pub fn manager(&self) -> &HashMap<String, PMConfig> {
        &self.manager
    }
}

impl FromStr for Config {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toml_parse_pmconfig() {
        let pm_config: PMConfig = toml::from_str(
            r#"
install = "install"
upgrade = { template = "upgrade -y" }
remove = "remove""#,
        )
        .unwrap();
        assert_eq!(Command::Simple("install".to_string()), pm_config.install);
        assert_eq!(
            Command::Inline(TemplateCommand {
                template: "upgrade -y".to_string()
            }),
            pm_config.upgrade
        );
        assert_eq!(Command::Simple("remove".to_string()), pm_config.remove);
    }

    #[test]
    fn test_toml_parse_config() {
        let config: Config = toml::from_str(
            r#"[manager.apt]
install = "install"
upgrade = { template = "upgrade -y" }
remove = "remove""#,
        )
        .unwrap();
        assert_eq!(
            Some(&PMConfig {
                install: Command::Simple("install".to_string()),
                upgrade: Command::Inline(TemplateCommand {
                    template: "upgrade -y".to_string()
                }),
                remove: Command::Simple("remove".to_string()),
            }),
            config.manager.get("apt")
        );
    }
}
