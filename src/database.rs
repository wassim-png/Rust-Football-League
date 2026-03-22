use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::fs;
use std::sync::Arc;

pub struct Database {
    pub conn: Arc<Connection>,
}

#[derive(Deserialize)]
struct JoueurLibreRow {
    #[allow(dead_code)]
    id: i32,
    nom: String,
    age: i32,
    numero: i32,
    poste: String,
    pied: String,
    potentiel: i32,
    reputation: i32,
    valeur_marche_eur: i64,
    salaire_semaine_eur: i64,
    fin_contrat: Option<String>,
    #[allow(dead_code)]
    nationalite: Option<String>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        let schema = fs::read_to_string("db/schema.sql").expect("Failed to read schema.sql");
        conn.execute_batch(&schema)?;
        println!("Database initialized successfully with schema.");

        seed_joueurs_libres(&conn);

        Ok(Self {
            conn: Arc::new(conn),
        })
    }
}

/// Insère les joueurs libres (club_id = NULL) depuis le CSV si aucun n'existe encore.
fn seed_joueurs_libres(conn: &Connection) {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM joueurs WHERE club_id IS NULL", [], |r| r.get(0))
        .unwrap_or(0);

    if count > 0 {
        return;
    }

    let mut rdr = match csv::Reader::from_path("db/data/joueurs_libres.csv") {
        Ok(r) => r,
        Err(e) => {
            println!("Impossible de lire joueurs_libres.csv : {}", e);
            return;
        }
    };

    let mut nb = 0;
    for result in rdr.deserialize::<JoueurLibreRow>() {
        match result {
            Ok(j) => {
                let res = conn.execute(
                    "INSERT INTO joueurs (club_id, nom, age, numero, poste, pied, potentiel,
                                         reputation, valeur_marche_eur, salaire_semaine_eur, fin_contrat)
                     VALUES (NULL, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                    rusqlite::params![
                        j.nom, j.age, j.numero, j.poste, j.pied,
                        j.potentiel, j.reputation, j.valeur_marche_eur,
                        j.salaire_semaine_eur, j.fin_contrat
                    ],
                );
                if res.is_ok() { nb += 1; }
            }
            Err(e) => println!("Erreur ligne CSV joueurs_libres : {}", e),
        }
    }
    println!("Joueurs libres seedés : {}", nb);
}
