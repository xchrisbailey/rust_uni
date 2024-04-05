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
        Commands::Delete { id } => todo!(),
        Commands::Complete { id } => todo!(),
        Commands::All => todo!(),
        Commands::View { id } => todo!(),
    }

    Ok(())
}

fn add_task(conn: &Connection, task: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (task, status) VALUES (?1, ?2)",
        (task, 0),
    )?;
    Ok(())
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
