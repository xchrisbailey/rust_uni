use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = match setup_database() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error setting up database: {}", e);
            return Ok(());
        }
    };

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { task } => add_task(&conn, task)?,
        Commands::Delete { id } => delete_task(&conn, *id)?,
        Commands::Complete { id } => complete_task(&conn, *id)?,
        Commands::All => get_all_tasks(&conn)?,
        Commands::View { id } => todo!(),
    }

    Ok(())
}

// Add a task to the database
fn add_task(conn: &Connection, task: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (task, status) VALUES (?1, ?2)",
        (task, 0),
    )?;
    Ok(())
}

// Delete a task from the database
fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

// Mark a task as complete
fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET status = 1 WHERE id = ?1", [id])?;
    Ok(())
}

// Get all tasks from the database
fn get_all_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            task: row.get(1)?,
            status: row.get(2)?,
        })
    })?;
    for task in task_iter {
        let Task { id, task, status } = task?;
        println!(
            "{}: {} - {}",
            id,
            task,
            match status {
                true => "finsihed",
                false => "pending",
            }
        );
    }
    Ok(())
}

#[derive(Debug)]
struct Task {
    id: i32,
    task: String,
    status: bool,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    Delete { id: i32 },
    Complete { id: i32 },
    All,
    View { id: i32 },
}

// Create a new database and table for tasks
fn setup_database() -> Result<Connection> {
    let conn = rusqlite::Connection::open("./tasks.db")?;
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
