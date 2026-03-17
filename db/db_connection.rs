use rusqlite::{Connection, Result};

pub struct DbConnection;

impl DbConnection {
    pub fn get_connection() -> Result<Connection> {
        Connection::open("football_manager.db")
    }
}