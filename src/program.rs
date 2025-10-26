use std::cell::OnceCell;

use crate::config::Config;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Program {
    connection: OnceCell<Connection>,
    config: Config,
}

impl Program {

}