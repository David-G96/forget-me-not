use anyhow::Ok;
use anyhow::Result;
use anyhow::anyhow;
use chrono::DateTime;
use chrono::Utc;
use rusqlite::Connection;
use rusqlite::params;

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


/// create all tables
pub fn init(conn: & Connection) -> Result<()> {
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
pub fn get_all_packages(conn: & Connection) -> Result<Vec<Package>> {
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

    let mut packages = Vec::new();
    for package in packages_iter {
        packages.push(package?);
    }

    Ok(packages)
}

pub fn insert_package(
    conn: &Connection,
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

pub fn insert_package_version(conn: &Connection, package_id: i64, version: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO PackageVersions (package_id, version) VALUES (?1, ?2)",
        params![package_id, version],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_installation(
    conn: &Connection,
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

pub const CREATE_TABLE_PACKAGES: &str = r#"CREATE TABLE IF NOT EXISTS Packages (
    package_id INTEGER PRIMARY KEY,
    package_name TEXT NOT NULL,
    source TEXT NOT NULL,
    description TEXT,
    UNIQUE (package_name, source)
);"#;

pub const CREATE_TABLE_PACKAGE_VERSION: &str = r#"CREATE TABLE IF NOT EXISTS PackageVersions (
    version_id INTEGER PRIMARY KEY,
    package_id INTEGER NOT NULL,
    version TEXT NOT NULL,
    FOREIGN KEY (package_id) REFERENCES Packages(package_id),
    UNIQUE (package_id, version)
);"#;

pub const CREATE_TABLE_INSTALLATIONS: &str = r#"CREATE TABLE IF NOT EXISTS Installations (
    installation_id INTEGER PRIMARY KEY,
    version_id INTEGER NOT NULL,
    action TEXT NOT NULL CHECK(action IN ('install', 'uninstall')),
    timestamp TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES PackageVersions(version_id)
);"#;

pub const CREATE_TABLE_TAGS: &str = r#"CREATE TABLE IF NOT EXISTS Tags (
    tag_id INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL UNIQUE
);"#;

pub const CREATE_TABLE_PACKAGE_TAGS: &str = r#"CREATE TABLE IF NOT EXISTS PackageTags (
    package_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (package_id) REFERENCES Packages(package_id),
    FOREIGN KEY (tag_id) REFERENCES Tags(tag_id),
    PRIMARY KEY (package_id, tag_id)
);"#;
