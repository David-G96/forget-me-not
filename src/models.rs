use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::Utc;
use log::info;
use rusqlite::Connection;
use rusqlite::params;
use semver::Op;

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
pub struct PackageTag {
    pub package_id: i64,
    pub tag_id: i64,
}
#[derive(Debug)]
pub struct PackageData {
    pub package_name: String,
    pub source: String,
    pub description: Option<String>,
    pub version: String,
    pub installation: String,
    pub tag: Option<String>,
}

pub fn get_all_package_data(conn: &mut Connection) -> Result<Vec<PackageData>> {
    let mut stmt = conn.prepare(
        "SELECT 
            p.package_name,
            p.source,
            p.description,
            GROUP_CONCAT(DISTINCT pv.version),
            GROUP_CONCAT(DISTINCT i.action),
            GROUP_CONCAT(DISTINCT i.timestamp),
            GROUP_CONCAT(DISTINCT t.tag_name)
        FROM Packages AS p
        LEFT JOIN PackageVersions AS pv ON p.package_id = pv.package_id
        LEFT JOIN Installations AS i ON pv.version_id = i.version_id
        LEFT JOIN PackageTags AS pt ON p.package_id = pt.package_id
        LEFT JOIN Tags AS t ON pt.tag_id = t.tag_id
        GROUP BY p.package_id
        ORDER BY p.package_name",
    )?;

    let query_iter = stmt.query_map([], |row| {
        let package_name = row.get(0)?;
        let source = row.get(1)?;
        let description = row.get(2)?;
        let version = row.get(3)?;
        let action: String = row.get(4)?;
        let timestamp: String = row.get(5)?;
        let tag_name = row.get(6)?;
        let data = rusqlite::Result::Ok(PackageData {
            package_name,
            source,
            description,
            version,
            installation: format!("{}:{}", action, timestamp),
            tag: tag_name,
        });
        data
    })?;

    let mut vec = Vec::new();

    for row in query_iter {
        vec.push(row?);
    }

    Ok(vec)
}

/// create all tables
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        &[
            CREATE_TABLE_PACKAGES,
            CREATE_TABLE_INSTALLATIONS,
            CREATE_TABLE_PACKAGE_TAGS,
            CREATE_TABLE_PACKAGE_VERSION,
            CREATE_TABLE_TAGS,
        ]
        .join("\n"),
    )
    .map_err(|e: rusqlite::Error| anyhow!("cannot init database table: {}", e))
}

pub fn get_all_packages(conn: &Connection) -> Result<Vec<Package>> {
    let mut stmt =
        conn.prepare("SELECT package_id, package_name, source, description FROM Packages")?;
    let packages_iter = stmt.query_map([], |row| {
        rusqlite::Result::Ok(Package {
            package_id: row.get(0)?,
            package_name: row.get(1)?,
            source: row.get(2)?,
            description: row.get(3)?,
        })
    })?;

    let packages: Vec<Package> = packages_iter.collect::<Result<Vec<_>, _>>()?;
    Ok(packages)
}

pub fn insert_package(
    conn: &mut Connection,
    package_name: &str,
    source: &str,
    description: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO Packages (package_name, source, description) VALUES (?1, ?2, ?3)",
        params![package_name, source, description],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_package_version(
    conn: &mut Connection,
    package_id: i64,
    version: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO PackageVersions (package_id, version) VALUES (?1, ?2)",
        params![package_id, version],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_installation(
    conn: &mut Connection,
    version_id: i64,
    action: &str,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Installations (version_id, action, timestamp) VALUES (?1, ?2, ?3)",
        params![version_id, action, timestamp.to_string()],
    )?;

    Ok(())
}

/// 主键package_id
pub const CREATE_TABLE_PACKAGES: &str = r#"CREATE TABLE IF NOT EXISTS Packages (
    package_id INTEGER PRIMARY KEY,
    package_name TEXT NOT NULL,
    source TEXT NOT NULL,
    description TEXT,
    UNIQUE (package_name, source)
);"#;

/// 主键version_id, 关联package_id
pub const CREATE_TABLE_PACKAGE_VERSION: &str = r#"CREATE TABLE IF NOT EXISTS PackageVersions (
    version_id INTEGER PRIMARY KEY,
    package_id INTEGER NOT NULL,
    version TEXT NOT NULL,
    FOREIGN KEY (package_id) REFERENCES Packages(package_id),
    UNIQUE (package_id, version)
);"#;

/// 主键installation_id, 关联version_id
pub const CREATE_TABLE_INSTALLATIONS: &str = r#"CREATE TABLE IF NOT EXISTS Installations (
    installation_id INTEGER PRIMARY KEY,
    version_id INTEGER NOT NULL,
    action TEXT NOT NULL CHECK(action IN ('install', 'uninstall')),
    timestamp TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES PackageVersions(version_id)
);"#;

/// 主键tag_id
pub const CREATE_TABLE_TAGS: &str = r#"CREATE TABLE IF NOT EXISTS Tags (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE
);"#;

/// 关联package_id, 关联tag_id
pub const CREATE_TABLE_PACKAGE_TAGS: &str = r#"CREATE TABLE IF NOT EXISTS PackageTags (
    package_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (package_id) REFERENCES Packages(package_id),
    FOREIGN KEY (tag_id) REFERENCES Tags(tag_id),
    PRIMARY KEY (package_id, tag_id)
);"#;
