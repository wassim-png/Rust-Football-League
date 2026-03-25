use rusqlite::{Connection, Result};
use std::fs;
use std::sync::Arc;
use std::process::Command;

pub struct Database {
    pub conn: Arc<Connection>,
}


impl Database{
    pub fn new(db_path: &str) -> Result<Self> {
         Self::init_db(db_path).expect("Failed to initialize database");
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



 fn init_db(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
   
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(r#"
rm -f db/simulation.db
sqlite3 db/simulation.db < db/schema.sql

sqlite3 db/simulation.db <<'EOFSQL'
.mode csv
.import --skip 1 db/data/competitions.csv competitions
.import --skip 1 db/data/info_club.csv info_club
.import --skip 1 db/data/joueurs.csv joueurs
.import --skip 1 db/data/saisons.csv saisons
.import --skip 1 db/data/clubs.csv clubs
.import --skip 1 db/data/saison_club.csv saison_club
.import --skip 1 db/data/etat_club_saison.csv etat_club_saison
.import --skip 1 db/data/primes_classement_saison.csv primes_classement_saison
.import --skip 1 db/data/stats_joueurs.csv attributs_joueur_saison
.import --skip 1 db/data/stats_gardiens.csv attributs_gardien_saison
INSERT INTO joueurs_libres (joueur_id) VALUES 
(396), (397), (398), (399), (400), 
(401), (402), (403), (404), (405), 
(406), (407), (408), (409), (410), 
(411), (412), (413), (414);
EOFSQL
"#)
        .output()?;

    if output.status.success() {
        println!("Base de données initialisée avec succès !");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Erreur d'initialisation : {}", error);
        return Err(error.into());
    }

    Ok(())
}

}
