use rusqlite::{params, Connection, Result};
use tracing::{error, info};
use crate::utils::Item;
pub fn insert(item: Item) -> Result<()> {
    let conn = Connection::open("../database.sqlite3")?;

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id    TEXT PRIMARY KEY,
            name  TEXT NOT NULL,
            proteins FLOAT,
            carbohydrates FLOAT,
            total_calories FLOAT,
            total_fats FLOAT
        )",
        (), // empty list of parameters.
    ){
        Ok(_) => info!("created successfully"),
        Err(err) => error!("created failed: {}", err)
    };

    match conn.execute(
        "INSERT INTO items (id, name, proteins, carbohydrates, total_calories, total_fats) \
        VALUES (?, ?, ?, ?, ?, ?)",
        (&item.id, &item.name, &item.proteins, &item.carbohydrates, &item.total_calories, &item.total_fats),
    ){
        Ok(_) => info!("insert successfully"),
        Err(e) => error!("insert failed: {}", e),
    };
    //
    Ok(())
}

pub fn read(id: String) -> Result<Vec<Item>> {
    let conn = Connection::open("../database.sqlite3")?;
    let mut stmt = conn.prepare("SELECT * FROM items")?;
    let iter = stmt.query_map([], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?,
            proteins: row.get(2)?,
            carbohydrates: row.get(3)?,
            total_calories: row.get(4)?,
            total_fats: row.get(5)?
        })
    }).expect("Error reading items");
    //
    let mut items: Vec<Item> = Vec::new();
    for item in iter {
        items.push(item?);
    }
    Ok(items)
}

pub fn update(item: Item) -> Result<()> {
    let conn = Connection::open("../database.sqlite3")?;

    match conn.execute(
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
    ){
        Ok(_) => info!("updated successfully"),
        Err(e) => error!("updated failed: {}", e),
    };
    //
    Ok(())
}

pub fn delete(id: String) -> Result<()> {
    let conn = Connection::open("../database.sqlite3")?;

    match conn.execute(
        "DELETE FROM `items`
        WHERE `id` = ?", params![&id],
    ) {
        Ok(_) => info!("deleted successfully"),
        Err(e) => error!("delete failed: {}", e)
    };
    Ok(())
}
