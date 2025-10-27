use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Package
#[derive(Debug)]
pub struct Package {
    pub package_id: i64,
    pub package_name: String,
    pub source: String,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct PackageVersion {
    pub version_id: i64,
    pub package_id: i64,
    pub version: String,
}

#[derive(Debug)]
pub struct Installation {
    pub installation_id: i64,
    pub version_id: i64,
    pub action: String,
    pub timestamp: DateTime<Utc>,
}

// 4. 对应 Tags 表
#[derive(Debug)]
pub struct Tag {
    pub tag_id: i64,
    pub tag_name: String,
}

// 5. 对应 PackageTags 连接表
#[derive(Debug)]
#[deprecated]
pub struct PackageTag {
    pub package_id: i64,
    pub tag_id: i64,
}

/// The collected, total package data
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageData {
    pub package_name: String,
    pub source: String,
    pub description: Option<String>,
    pub version: String,
    pub installation: String,
    pub tags: Option<Vec<String>>,
}

pub struct SimplePackageData {
    pub name: String,
    pub source: String,
    pub description: Option<String>,
    pub installation: String,
}
