use rusqlite::{Connection, Result, params};
use tracing::{info, warn};
use crate::utils::Item;

const DB_PATH: &str = "database.sqlite3";

/// Inserts an item into the database. Takes a reference to avoid consuming the item.
/// # Arguments
/// - item: `&Item`
pub fn insert(item: &Item) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Create table only if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id    TEXT PRIMARY KEY,
            name  TEXT NOT NULL,
            proteins FLOAT,
            carbohydrates FLOAT,
            total_calories FLOAT,
            total_fats FLOAT
        )",
        (),
    )?;

    conn.execute(
        "INSERT OR REPLACE INTO items (id, name, proteins, carbohydrates, total_calories, total_fats)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            &item.id,
            &item.name,
            &item.proteins,
            &item.carbohydrates,
            &item.total_calories,
            &item.total_fats
        ],
    )?;

    info!("Successfully inserted item with id: {}", item.id);
    Ok(())
}

/// Gets a specified item by id.
/// # Arguments
/// - id: `&str`
pub fn read(id: &str) -> Result<Vec<Item>> {
    let conn = Connection::open(DB_PATH)?;
    // Prepare a statement with a WHERE clause to fetch only the desired item.
    let mut stmt = conn.prepare("SELECT * FROM items WHERE id = ?1")?;

    let iter = stmt.query_map(params![id], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            proteins: row.get(2)?,
            carbohydrates: row.get(3)?,
            total_calories: row.get(4)?,
            total_fats: row.get(5)?,
        })
    })?;

    let mut items = Vec::new();
    for item in iter {
        items.push(item?);
    }
    Ok(items)
}

/// Updates an item in the database. Takes a reference.
/// # Arguments
/// - item: `&Item`
pub fn update(item: &Item) -> Result<u32> {
    let conn = Connection::open(DB_PATH)?;

    let rows_affected = conn.execute(
        "UPDATE `items`
        SET `name` = ?1, `proteins` = ?2, `carbohydrates` = ?3, `total_calories` = ?4, `total_fats` = ?5
        WHERE `id` = ?6",
        params![
            &item.name,
            &item.proteins,
            &item.carbohydrates,
            &item.total_calories,
            &item.total_fats,
            &item.id,
        ],
    )?;

    if rows_affected == 0 {
        warn!("Update did not affect any rows for item id: {}", item.id);
        return Ok(404);
    }
    info!("Successfully updated item id: {}", item.id);
    Ok(200)
}

/// Deletes a specified item by id.
/// # Arguments
/// - id: `&str`
pub fn delete(id: &str) -> Result<u32> {
    let conn = Connection::open(DB_PATH)?;

    let rows_affected = conn.execute("DELETE FROM items WHERE `id` = ?1", params![id])?;

    if rows_affected == 0 {
        warn!("Delete did not affect any rows for item id: {}", id);
        return Ok(404);
    }
    info!("Successfully deleted item with id: {}", id);
    Ok(200)
}
