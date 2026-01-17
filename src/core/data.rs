use chrono::{DateTime, Utc};
use color_eyre::{Result as Res, eyre::ensure};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    cell::{LazyCell, OnceCell},
    collections::HashMap,
    fs::{File, read_to_string},
    path::{self, Path, PathBuf},
};

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

#[derive(Debug, Default)]
pub struct DataBase {
    pub data: HashMap<u32, RecordData>,
}

impl DataBase {
    pub fn from_vec(data: Vec<RecordData>) -> Self {
        let mut map = HashMap::new();
        for d in data {
            map.insert(d.id, d);
        }
        Self { data: map }
    }

    pub fn from_json_db(path: &Path) -> Res<Self> {
        ensure!(path.exists(), "path does not exist");
        let db_file = read_to_string(path)?;
        let data: Vec<RecordData> = serde_json::from_str(&db_file)?;
        Ok(Self::from_vec(data))
    }
}

#[derive(Debug)]
pub struct DataManager {
    pub db: Res<DataBase>,
    pub staged: DataBase,
}

impl DataManager {
    pub fn new(path: &Path) -> Self {
        Self {
            db: DataBase::from_json_db(path),
            staged: DataBase::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_db_de() -> Res<()> {
        let str = r#"{
  "id": 0,
  "name": "pkg1",
  "version": {
    "Sematic": "1.21.0"
  },
  "installationDate": "2026-01-17T00:17:23.283758Z",
  "location": "/a/b/c",
  "source": "org.wonderland",
  "tags": [
    "wtf",
    "rusty",
    "foo",
    "bar"
  ],
  "description": "What is this? I don't know."
}"#;

        let d: Vec<RecordData> = serde_json::from_str(str)?;
        Ok(())
    }

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
