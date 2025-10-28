use rusqlite::{Connection, params};

use crate::simpledata::data::SimplePackageData;

const PACKAGE_TABLE_NAME: &str = "Packages";

const CREATE_TABLE: &str = r#"CREATE TABLE Packages(
ID INTEGER PRIMARY KEY,
Name TEXT NOT NULL,
Source TEXT NOT NULL,
Description TEXT,
Installation TEXT
)"#;

const QUERY_ALL_PACKAGE: &str = r#"SELECT * FROM Packages"#;

const INSERT_PACKAGE: &str = r#"INSERT INTO Packages (Name, Source, Description, Installation) 
                                   VALUES (?1, ?2, ?3, ?4)"#;
const DELETE_PACKAGES: &str = r#"DELETE FROM Packages"#;

/// create table if not exists
pub fn try_create_table(conn: &mut Connection) -> Result<(), String> {
    match conn.table_exists(None, PACKAGE_TABLE_NAME) {
        Ok(table_exists) => {
            // table does not exists, create it
            if !table_exists {
                let res = conn.execute(CREATE_TABLE, params![]);
                match res {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e.to_string()),
                }
            }
            // table already exists
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

/// query all packages from table Packages
pub fn try_list_all(conn: &mut Connection) -> Result<Vec<SimplePackageData>, String> {
    debug_assert!(
        Ok(true) == conn.table_exists(None, PACKAGE_TABLE_NAME),
        "Internal logic error: table {} does not exists",
        PACKAGE_TABLE_NAME
    );
    let mut stmt = conn.prepare(QUERY_ALL_PACKAGE).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(SimplePackageData {
                id: row.get(0)?,
                name: row.get(1)?,
                source: row.get(2)?,
                description: row.get(3)?,
                installation: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let results: Result<Vec<SimplePackageData>, String> = rows
        .map(|row_result| row_result.map_err(|e| e.to_string()))
        .collect();
    results
}

/// try to insert one package data
/// # Note
/// ignores the id of the package
pub fn try_insert(
    conn: &mut Connection,
    simple_package: SimplePackageData,
) -> Result<usize, String> {
    debug_assert!(
        Ok(true) == conn.table_exists(None, PACKAGE_TABLE_NAME),
        "Internal logic error: table {} does not exists",
        PACKAGE_TABLE_NAME
    );

    conn.execute(
        INSERT_PACKAGE,
        params![
            simple_package.name,
            simple_package.source,
            simple_package.description,
            simple_package.installation
        ],
    )
    .map_err(|e| e.to_string())
}

/// clear all packages
pub fn try_clear_packages(conn: &mut Connection) -> Result<usize, String> {
    conn.execute(DELETE_PACKAGES, params![])
        .map_err(|e| e.to_string())
}

const DELETE_PACKAGE_BY_NAME: &str = r#"DELETE FROM Packages WHERE Name = ?1"#;

pub fn try_delete_package_name(conn: &mut Connection, package_name: &str) -> Result<usize, String> {
    conn.execute(DELETE_PACKAGE_BY_NAME, params![package_name])
        .map_err(|e| e.to_string())
}
