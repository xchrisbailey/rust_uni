use rusqlite::{Connection, Result};

// Add a task to the database
pub fn add_task(conn: &Connection, task: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (task, status) VALUES (?1, ?2)",
        (task, 0),
    )?;
    Ok(())
}

// Delete a task from the database
pub fn delete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
    Ok(())
}

// Mark a task as complete
pub fn complete_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET status = 1 WHERE id = ?1", [id])?;
    Ok(())
}

// Get all tasks from the database
pub fn get_all_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            task: row.get(1)?,
            status: row.get(2)?,
        })
    })?;
    for task in task_iter {
        task?.print();
    }
    Ok(())
}

// Find single task by id
pub fn get_task_by_id(conn: &Connection, id: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1")?;
    let task = stmt.query_row([id], |row| {
        Ok(Task {
            id: row.get(0)?,
            task: row.get(1)?,
            status: row.get(2)?,
        })
    })?;
    task.print();
    Ok(())
}

#[derive(Debug)]
struct Task {
    id: i32,
    task: String,
    status: bool,
}

impl Task {
    // fn new(id: i32, task: String, status: bool) -> Task {
    //     Task { id, task, status }
    // }

    fn print(&self) {
        println!(
            "{}: {} - {}",
            self.id,
            self.task,
            match self.status {
                true => "finsihed",
                false => "pending",
            }
        );
    }
}
