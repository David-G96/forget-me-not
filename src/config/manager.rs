use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

/// represents a template command
/// e.g. install = { template = "install --user {package_name} --assumeyes" }
#[derive(Debug, Deserialize, PartialEq)]
pub struct TemplateCommand {
    pub template: String,
}

// 使用 #[serde(untagged)] 来告诉 Serde 尝试按顺序匹配每一个变体，
// 直到成功为止，而不需要一个额外的标签字段来区分它们。
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Command {
    // Serde 会首先尝试将数据解析为 Inline 变体（一个表/结构体）
    Template(TemplateCommand),
    /// Represents a simple command, with only one String
    /// the args will append after it with a space
    Simple(String),
}

impl Command {
    /// support for command format\
    /// e.g.
    /// package_name = "abc" and "install {package_name}" will be formatted into "install abc"
    /// # Supported placeholders:
    /// package_name
    pub fn format(&self, package_name: &str) -> String {
        match self {
            Self::Template(template) => template.template.replace("{package_name}", package_name),
            Self::Simple(s) => {
                format!("{} {}", s, package_name)
            }
        }
    }
}

/// config for package managers, e.g. apt, dnf
/// this should contains commands for install, upgrade, remove .etc
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct SingleManagerConfig {
    install: Command,
    upgrade: Command,
    remove: Command,
}

/// config for all package managers
#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct ManagerConfigs {
    manager: HashMap<String, SingleManagerConfig>,
}

impl ManagerConfigs {
    pub fn config_of(&self, package_manager_name: &str) -> Option<&SingleManagerConfig> {
        self.manager.get(package_manager_name)
    }
}

impl FromStr for ManagerConfigs {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

impl Default for ManagerConfigs {
    fn default() -> Self {
        let mut manager = HashMap::new();
        manager.insert(
            "brew".to_string(),
            SingleManagerConfig {
                install: Command::Template(TemplateCommand {
                    template: "install {package_name}".to_string(),
                }),
                upgrade: Command::Template(TemplateCommand {
                    template: "upgrade {package_name}".to_string(),
                }),
                remove: Command::Template(TemplateCommand {
                    template: "remove {package_name".to_string(),
                }),
            },
        );
        Self { manager }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toml_parse_pmconfig() {
        let pm_config: SingleManagerConfig = toml::from_str(
            r#"
install = "install"
upgrade = { template = "upgrade -y" }
remove = "remove""#,
        )
        .unwrap();
        assert_eq!(Command::Simple("install".to_string()), pm_config.install);
        assert_eq!(
            Command::Template(TemplateCommand {
                template: "upgrade -y".to_string()
            }),
            pm_config.upgrade
        );
        assert_eq!(Command::Simple("remove".to_string()), pm_config.remove);
    }

    #[test]
    fn test_toml_parse_config() {
        let config: ManagerConfigs = toml::from_str(
            r#"[manager.apt]
install = "install"
upgrade = { template = "upgrade -y" }
remove = "remove""#,
        )
        .unwrap();
        assert_eq!(
            Some(&SingleManagerConfig {
                install: Command::Simple("install".to_string()),
                upgrade: Command::Template(TemplateCommand {
                    template: "upgrade -y".to_string()
                }),
                remove: Command::Simple("remove".to_string()),
            }),
            config.config_of("apt")
        );
    }
}
