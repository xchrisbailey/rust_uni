use homedir::get_my_home;
use rusqlite::{Connection, Result};

// Create a new database and table for tasks
pub fn setup_database() -> Result<Connection> {
    let conn: Connection;

    match get_db_path() {
        Ok(path) => conn = rusqlite::Connection::open(path)?,
        Err(e) => {
            println!("Error getting database path: {}", e);
            std::process::exit(1)
        }
    }

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
id INTEGER PRIMARY KEY,
task TEXT NOT NULL,
status BOOLEAN NOT NULL DEFAULT 0
)",
        [],
    )?;

    Ok(conn)
}

// Get the path to the database
fn get_db_path() -> Result<String, std::io::Error> {
    if let Ok(path) = get_my_home() {
        Ok(format!("{}/task.db", path.unwrap().display()))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Error getting home directory",
        ))
    }
}
