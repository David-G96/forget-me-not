use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::config::config::Config;

/// the essential context of an app
#[derive(Debug)]
struct AppContext {
    config: Config,
    // 使用 Arc 和 Mutex 来安全地共享 DB 连接并满足 &mut 要求
    db_conn: Arc<Mutex<Connection>>,
}
