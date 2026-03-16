use rusqlite::{Connection, Result};
use std::fs;

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Read the schema file
    let schema = fs::read_to_string("db/schema.sql").expect("Failed to read schema.sql");
    
    // Execute the schema to create tables
    conn.execute_batch(&schema)?;
    
    println!("Database initialized successfully with schema.");
    Ok(conn)
}

pub fn seed_database(conn: &Connection) -> Result<()> {
    // TODO: Implement CSV reading using the `csv` crate
    // and insert data into `joueurs` and `clubs` tables.
    println!("Database seeding placeholder.");
    Ok(())
}
