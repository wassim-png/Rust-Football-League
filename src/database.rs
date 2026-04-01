use rusqlite::{Connection, Result};
use std::fs;
use std::sync::Arc;
use std::path::Path;

pub struct Database {
    pub conn: Arc<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        // Supprimer l'ancienne DB si elle existe, pour repartir proprement
        if Path::new(db_path).exists() {
            fs::remove_file(db_path).expect("Impossible de supprimer l'ancienne DB");
        }

        let conn = Connection::open(db_path)?;

        // Créer les tables depuis le schéma
        let schema = fs::read_to_string("db/schema.sql").expect("Failed to read schema.sql");
        conn.execute_batch(&schema)?;

        // Désactiver les FK pendant l'import (le schéma les active)
        conn.execute_batch("PRAGMA foreign_keys = OFF;")?;

        // Importer les données CSV (dans l'ordre des tables)
        // Le format est (csv_path, table_name, colonnes_db_dans_l'ordre_du_csv)
        let imports: Vec<(&str, &str, Vec<&str>)> = vec![
            ("db/data/competitions.csv", "competitions",
                vec!["id", "nom", "nb_equipes"]),
            ("db/data/saisons.csv", "saisons",
                vec!["id", "competition_id", "annee"]),
            ("db/data/clubs.csv", "clubs",
                vec!["id", "nom", "nom_court", "reputation", "budget_eur",
                     "revenu_par_journee_eur", "points", "buts_marques",
                     "buts_encaisses", "avantage_domicile"]),
            ("db/data/info_club.csv", "info_club",
                vec!["info_club_id", "club_id", "nom_stade", "stade_capacite",
                     "url_logo", "url_stade", "nom_meilleur_buteur"]),
            ("db/data/saison_club.csv", "saison_club",
                vec!["saison_id", "club_id"]),
            ("db/data/etat_club_saison.csv", "etat_club_saison",
                vec!["club_id", "saison_id", "moral", "reputation"]),
            ("db/data/joueurs.csv", "joueurs",
                vec!["id", "club_id", "nom", "age", "numero", "poste", "pied",
                     "potentiel", "reputation", "valeur_marche_eur",
                     "salaire_semaine_eur", "fin_contrat"]),
            ("db/data/stats_joueurs.csv", "attributs_joueur_saison",
                vec!["joueur_id", "saison_id", "vitesse", "tir", "passe",
                     "dribble", "defense", "physique", "forme", "moral",
                     "nationalite", "note_actuelle"]),
            ("db/data/stats_gardiens.csv", "attributs_gardien_saison",
                vec!["joueur_id", "saison_id", "plongeon", "jeu_a_la_main",
                     "passe", "reflexe", "vitesse", "position", "forme",
                     "moral", "nationalite", "note_actuelle"]),
            ("db/data/primes_classement_saison.csv", "primes_classement_saison",
                vec!["saison_id", "rang", "montant_eur"]),
        ];

        for (csv_path, table_name, columns) in &imports {
            Self::import_csv(&conn, csv_path, table_name, columns)?;
        }

        // Import spécial : joueurs_libres.csv contient des joueurs complets (sans club)
        // On les insère d'abord dans 'joueurs', puis on référence leurs IDs dans 'joueurs_libres'
        

        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        println!("Database initialized successfully with schema.");
        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    /// Importe un fichier CSV dans une table SQLite en utilisant
    /// les noms de colonnes spécifiés (par position dans le CSV).
    fn import_csv(
        conn: &Connection,
        csv_path: &str,
        table_name: &str,
        columns: &[&str],
    ) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(csv_path)
            .unwrap_or_else(|e| panic!("Impossible de lire {} : {}", csv_path, e));

        let nb_cols = columns.len();
        let col_names = columns.join(", ");
        let placeholders = vec!["?"; nb_cols].join(", ");
        let sql = format!(
            "INSERT OR IGNORE INTO {} ({}) VALUES ({})",
            table_name, col_names, placeholders
        );

        let mut stmt = conn.prepare(&sql)?;

        for result in reader.records() {
            let record = result.unwrap_or_else(|e| {
                panic!("Erreur lecture ligne dans {} : {}", csv_path, e)
            });

            let values: Vec<String> = record.iter().map(|v| v.to_string()).collect();
            let params: Vec<&dyn rusqlite::types::ToSql> = values
                .iter()
                .map(|v| v as &dyn rusqlite::types::ToSql)
                .collect();

            stmt.execute(params.as_slice())?;
        }

        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            [],
            |row| row.get(0),
        )?;
        println!("  > {} : {} lignes", table_name, count);

        Ok(())
    }

    /// Import spécial pour joueurs_libres.csv :
    /// Le CSV contient des joueurs complets (sans club).
    /// On les insère dans `joueurs` puis on les référence dans `joueurs_libres`.
    fn import_joueurs_libres(conn: &Connection) -> Result<()> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path("db/data/joueurs_libres.csv")
            .expect("Impossible de lire joueurs_libres.csv");

        let mut stmt_joueur = conn.prepare(
            "INSERT OR IGNORE INTO joueurs (id, nom, age, numero, poste, pied, potentiel, reputation, valeur_marche_eur, salaire_semaine_eur, fin_contrat)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )?;

        let mut stmt_libre = conn.prepare(
            "INSERT OR IGNORE INTO joueurs_libres (joueur_id) VALUES (?)"
        )?;

        let mut count = 0;
        for result in reader.records() {
            let record = result.expect("Erreur lecture joueurs_libres.csv");
            // Colonnes CSV: id, nom, age, numero, poste, pied, potentiel, reputation,
            //               valeur_marche_eur, salaire_semaine_eur, fin_contrat, nationalite
            let id: String = record[0].to_string();
            let nom: String = record[1].to_string();
            let age: String = record[2].to_string();
            let numero: String = record[3].to_string();
            let poste: String = record[4].to_string();
            let pied: String = record[5].to_string();
            let potentiel: String = record[6].to_string();
            let reputation: String = record[7].to_string();
            let valeur: String = record[8].to_string();
            let salaire: String = record[9].to_string();
            let fin_contrat: String = record[10].to_string();

            // Insérer dans joueurs (sans club_id => NULL)
            stmt_joueur.execute(rusqlite::params![
                id, nom, age, numero, poste, pied, potentiel, reputation,
                valeur, salaire, fin_contrat
            ])?;

            // Référencer dans joueurs_libres
            stmt_libre.execute(rusqlite::params![id])?;
            count += 1;
        }

        println!("  > joueurs_libres : {} lignes", count);
        Ok(())
    }
}
