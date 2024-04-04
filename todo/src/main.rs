use clap::Parser;
use rusqlite::Result;

fn main() {
    if let Err(e) = setup_database() {
        eprintln!("Error setting up database: {}", e);
        std::process::exit(1);
    }

    let cli = Cli::parse();
    if let Some(task) = cli.task.as_deref() {
        println!("{}", task);
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    task: Option<String>,
}

fn setup_database() -> Result<()> {
    let conn = rusqlite::Connection::open("./tasks.db")?;

    conn.execute(
        "CREATE TABLE task (
id INTEGER PRIMARY KEY,
task TEXT NOT NULL,
done BOOLEAN NOT NULL DEFAULT 0
)",
        [],
    )?;

    Ok(())
}
