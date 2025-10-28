use serde::{Deserialize, Serialize};

use crate::config::manager::SingleManagerConfig;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    manager: HashMap<String, SingleManagerConfig>,
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
