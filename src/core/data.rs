use std::path::PathBuf;

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexibleVersion {
    Sematic(Version),
    Raw(String),
}

impl FlexibleVersion {
    pub fn parse(s: &str) -> Self {
        match Version::parse(s) {
            Ok(ver) => Self::Sematic(ver),
            Err(_) => Self::Raw(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    pub id: u32,
    pub name: String,
    pub version: Option<FlexibleVersion>,
    pub installation_date: Option<DateTime<Utc>>,
    #[serde(serialize_with = "serialize_unix_path")]
    pub location: Option<PathBuf>,
    pub source: Option<String>,
    pub tags: Vec<String>,
    /// 如果为 None，序列化时可以忽略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 自定义序列化函数：将路径中的反斜杠转换为正斜杠
fn serialize_unix_path<S>(path: &Option<PathBuf>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(path) = path {
        // 将路径转换为字符串
        let s = path.to_string_lossy();
        // 将 Windows 的 \\ 替换为 /
        let normalized = s.replace('\\', "/");
        serializer.serialize_str(&normalized)
    } else {
        serializer.serialize_none()
    }
}

impl RecordData {
    pub fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_record_data_ser() {
        let data = RecordData {
            id: 0,
            name: "pkg1".into(),
            version: FlexibleVersion::Sematic(Version::parse("1.21.0").unwrap()).into(),
            installation_date: Utc::now().into(),
            location: PathBuf::from("/a/b/c").into(),
            source: "org.wonderland".to_string().into(),
            tags: vec!["wtf", "rusty", "foo", "bar"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            description: "What is this? I don't know.".to_string().into(),
        };
        let res = serde_json::to_string_pretty(&data).unwrap();
        println!("pretty:\n{}", res);
        println!("plain:\n{}", serde_json::to_string(&data).unwrap());
    }
}
