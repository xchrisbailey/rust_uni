use colored::Colorize;
use rusqlite::Connection;

// Add a task to the database
pub fn add_task(conn: &Connection, task: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO tasks (task, status) VALUES (?1, ?2)",
        (task, 0),
    )?;
    Ok(())
}

// Delete a task from the database
pub fn delete_task(conn: &Connection, id: i32) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

// Mark a task as complete
pub fn complete_task(conn: &Connection, id: i32) -> rusqlite::Result<()> {
    conn.execute("UPDATE tasks SET status = 1 WHERE id = ?1", [id])?;
    Ok(())
}

// Get all tasks from the database
pub fn get_all_tasks(conn: &Connection) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            task: row.get(1)?,
            status: row.get(2)?,
        })
    })?;

    println!(
        "{}\t{}\t\t{}",
        "ID".bold(),
        "Status  ".bold(),
        "Task".bold(),
    );

    for task in task_iter {
        println!("{}\n", task?);
    }
    Ok(())
}

// Find single task by id
pub fn get_task_by_id(conn: &Connection, id: i32) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let task = stmt.query_row([id], |row| {
        Ok(Task {
            id: row.get(0)?,
            task: row.get(1)?,
            status: row.get(2)?,
        })
    })?;
    println!("{}", task);
    Ok(())
}

#[derive(Debug, Clone)]
struct Task {
    id: i32,
    task: String,
    status: bool,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.status {
            true => write!(
                f,
                "{}\t{}\t\t{}",
                self.id.to_string().blue().bold().strikethrough(),
                "finished".red().strikethrough(),
                self.task.blue().strikethrough(),
            ),
            false => write!(
                f,
                "{}\t{}\t\t{}",
                self.id.to_string().blue().bold(),
                "pending".yellow(),
                self.task.blue(),
            ),
        }
    }
}
