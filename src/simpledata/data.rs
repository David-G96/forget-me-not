use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimplePackageData {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub description: Option<String>,
    pub installation: Option<String>,
}
