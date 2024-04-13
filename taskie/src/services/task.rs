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
        print!("{}", task?.to_string());
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
    println!("{}", task.to_string());
    Ok(())
}

#[derive(Debug, Clone)]
struct Task {
    id: i32,
    task: String,
    status: bool,
}

// Implement the ToString trait for Task
impl ToString for Task {
    fn to_string(&self) -> String {
        match self.status {
            // if task is finished
            true => format!(
                "{}\t{}\t\t{}\n",
                self.id.to_string().blue().bold(),
                match self.status {
                    true => "finsihed".red(),
                    false => "pending".yellow(),
                },
                self.task.blue(),
            )
            .strikethrough()
            .to_string(),
            // if task is pending
            false => format!(
                "{}\t{}\t\t{}\n",
                self.id.to_string().blue().bold(),
                match self.status {
                    true => "finsihed".red(),
                    false => "pending ".yellow(),
                },
                self.task.blue(),
            ),
        }
    }
}
