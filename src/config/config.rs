use serde::Deserialize;

use crate::config::ManagerConfigs;

#[derive(Debug, Deserialize)]
pub struct Config {
    manager: ManagerConfigs,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let config_str = r#"[manager.apt]
install = "install"
upgrade = { template = "upgrade -y" }
remove = "remove"

[manager.flatpak]
install = { template = "install --user {package_name} --assumeyes" }
upgrade = "update"
remove = "remove"
"#;
    }
}
