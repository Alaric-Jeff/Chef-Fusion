use rusqlite::Connection;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    Mutex::new(Connection::open("chef_fusion.db").expect("Failed to open DB"))
});
