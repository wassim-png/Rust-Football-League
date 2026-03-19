use rusqlite::{Connection, Result};
use std::fs;
use std::sync::Arc;

pub struct Database {
    pub conn: Arc<Connection>,
}


impl Database{
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Read the schema file
        let schema = fs::read_to_string("db/schema.sql").expect("Failed to read schema.sql");
        
        // Execute the schema to create tables
        conn.execute_batch(&schema)?;
        
        println!("Database initialized successfully with schema.");
        Ok(Self {
            conn: Arc::new(conn), 
        })
    }
}


