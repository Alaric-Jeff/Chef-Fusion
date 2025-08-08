use rusqlite::Connection;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    Mutex::new(Connection::open("chef_fusion.db").expect("Failed to open DB"))
});

pub fn setup_database() {
    let conn = DB.lock().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS menu_table (
            menu_id        INTEGER PRIMARY KEY AUTOINCREMENT,
            menu_name      TEXT NOT NULL,
            menu_price     REAL NOT NULL,
            category       TEXT,
            inv_id         TEXT, 
            total_serving  INTEGER NOT NULL,
            date           TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).expect("Error in creating menu_table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS order_table (
            order_id   INTEGER PRIMARY KEY AUTOINCREMENT,
            menu_id    INTEGER NOT NULL,
            price      REAL NOT NULL DEFAULT 0,
            date       TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).expect("Error in creating order_table");
}
