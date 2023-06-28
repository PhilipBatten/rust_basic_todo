use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub fn db_setup() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn add_todo(title: String, description: String) -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "INSERT INTO todo (title, description) VALUES (?1, ?2)",
        [title, description],
    )?;

    Ok(())
}

pub fn read_todos() -> Result<Vec<Todo>> {
    let conn = Connection::open("todo.db")?;

    let mut stmt = conn.prepare("SELECT id, title, description FROM todo")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }

    Ok(todos)
}

pub fn remove_todo_by_id(id: i32) -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute("DELETE FROM todo WHERE id = ?1", [id])?;

    Ok(())
}