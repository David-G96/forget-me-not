use rusqlite::{Connection, Result};

pub const APP_DIR_NAME: &str = "forget-me-not";
pub const DB_NAME: &str = "todos.db";

#[derive(Debug)]
pub struct InstalledPackages {
    pub manager: String,
    pub package: String,
    pub version: Option<String>,
    pub install_date: String,
}
pub fn get_db_path() -> Option<std::path::PathBuf> {
    // 1. 獲取使用者資料目錄
    if let Some(mut path) = dirs::data_dir() {
        // 2. 為你的應用程式創建一個子目錄
        path.push(APP_DIR_NAME);
        // 3. 確保這個子目錄存在
        if std::fs::create_dir_all(&path).is_err() {
            // 如果無法創建，就返回 None
            return None;
        }

        // 4. 在子目錄中建立資料庫檔案的完整路徑
        path.push(DB_NAME);
        Some(path)
    } else {
        None
    }
}

pub fn create_table(conn: &mut Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS installed_packages (
    id INTEGER PRIMARY KEY,
    manager TEXT NOT NULL,
    package TEXT NOT NULL,
    version TEXT,
    install_date TEXT NOT NULL
);",
        (),
    )?;
    Ok(())
}

pub fn add_package(
    conn: &mut Connection,
    manager: String,
    package: String,
    version: String,
    install_date: String,
) -> Result<()> {
    conn.execute(
        "INSERT INTO installed_packages (manager, package, version, install_date)
VALUES (?1, ?2, ?3, ?4);",
        (manager, package, version, install_date),
    )?;
    Ok(())
}

pub fn query_all(conn: &mut Connection) -> Vec<InstalledPackages> {
    let mut stmt = conn.prepare("SELECT * FROM installed_packages").unwrap();
    let res = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0).unwrap();
            let manager: String = row.get(1).unwrap();
            let package: String = row.get(2).unwrap();
            let version: String = row.get(3).unwrap();
            let install_date: String = row.get(4).unwrap();

            let installed_pkg = InstalledPackages {
                manager,
                package,
                version: Some(version),
                install_date,
            };

            Ok(installed_pkg)
        })
        .unwrap();

    let pkgs: Vec<_> = res.map(|x| x.unwrap()).collect();
    pkgs
}

pub fn query_package(conn: &mut Connection, package: String) -> bool {
    let num = conn
        .execute(
            "SELECT id, manager, package FROM installed_packages
WHERE package = ?1;",
            (package,),
        )
        .unwrap();

    num != 0
}

pub fn remove_data(conn: &mut Connection, package: String) -> bool {
    let res = conn
        .execute(
            "DELETE FROM installed_packages
WHERE package = ?1;",
            ((package),),
        )
        .unwrap_or(0);
    res != 0
}

pub fn update_package(
    conn: &mut Connection,
    package: String,
    version: String,
    date: String,
) -> bool {
    conn.execute(
        "UPDATE installed_packages
SET version = ?1, install_date = ?2
WHERE package = ?3;",
        (version, date, package),
    )
    .unwrap_or(0)
        != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let path = get_db_path().unwrap();

        let mut conn = Connection::open(path).unwrap();
        create_table(&mut conn).unwrap();

        add_package(
            &mut conn,
            "flatpak".to_string(),
            "org.mozilla.Firefox".to_string(),
            "128.0.0".to_string(),
            "2025-08-15".to_string(),
        )
        .unwrap();

        {
            let mut stmt = conn.prepare("SELECT * FROM installed_packages").unwrap();
            let res = stmt
                .query_map([], |row| {
                    let id: i32 = row.get(0).unwrap();
                    let manager: String = row.get(1).unwrap();
                    let package: String = row.get(2).unwrap();
                    let version: String = row.get(3).unwrap();
                    let install_date: String = row.get(4).unwrap();

                    let installed_pkg = InstalledPackages {
                        manager,
                        package,
                        version: Some(version),
                        install_date,
                    };

                    Ok(installed_pkg)
                })
                .unwrap();

            println!("now ");
            for r in res {
                println!("{:?}", r.unwrap());
            }
        }

        let res = remove_data(&mut conn, "org.mozilla.Firefox".to_string());
        println!("is firefox dropped? {}", res);

        println!("now the table is ");
        let res = query_all(&mut conn);
        for r in res {
            println!("{:#?}", r);
        }
    }
}
