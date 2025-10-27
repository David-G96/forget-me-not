use std::{cell::OnceCell, str::FromStr, sync::OnceLock};

use crate::config::Config;
use rusqlite::Connection;

/// Program of forget-me-not, config and connection are both immutable once initialized
#[derive(Debug)]
pub struct Program {
    connection: OnceLock<Connection>,
    config: OnceLock<Config>,
}

impl Program {
    /// init the config with the specific config str.
    /// Will not reload the config if config already exists
    pub fn init_config(&mut self, config_str: &str) -> Result<(), toml::de::Error> {
        let config = Config::from_str(config_str)?;
        self.config.get_or_init(|| config);
        Ok(())
    }
}
