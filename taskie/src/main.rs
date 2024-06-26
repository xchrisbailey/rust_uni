mod database;
mod services;

use clap::{Parser, Subcommand};
use database::setup_database;
use rusqlite::Result;
use services::task::{add_task, complete_task, delete_task, get_all_tasks, get_task_by_id};

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
        Commands::View { id } => get_task_by_id(&conn, *id)?,
    }

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
