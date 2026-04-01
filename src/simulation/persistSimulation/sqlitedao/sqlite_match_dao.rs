use rusqlite::params;
use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::simulation::persistSimulation::dao::match_dao::MatchDao;
use rusqlite::OptionalExtension;
use crate::models::Match;
use crate::models::ResultatSimulationMatch;

pub struct SqliteMatchDao{
     pub conn: Arc<Connection>,
}


 

impl MatchDao for SqliteMatchDao {
    fn find_match_by_id(&self, match_id: i32) -> Result<Option<Match>, String> {
        
        let resultat = self.conn.query_row(
            "
            SELECT 
                m.id, m.journee, m.club_domicile_id, 
                cd.nom, cd.url_logo,
                m.club_exterieur_id, 
                ce.nom, ce.url_logo,
                m.date_coup_envoi, m.buts_domicile, m.buts_exterieur
            FROM matchs m
            JOIN clubs cd ON m.club_domicile_id = cd.id
            JOIN clubs ce ON m.club_exterieur_id = ce.id
            WHERE m.id = ?1
            ",
            rusqlite::params![match_id],
            |row| {
                
                Ok(Match {
                    id: row.get(0)?,
                    journee: row.get(1)?,
                    club_domicile_id: row.get(2)?,
                    club_domicile_nom: row.get(3)?,
                    club_domicile_logo: row.get(4)?,
                    club_exterieur_id: row.get(5)?,
                    club_exterieur_nom: row.get(6)?,
                    club_exterieur_logo: row.get(7)?,
                    date_coup_envoi: row.get(8)?,
                    buts_domicile: row.get(9)?,
                    buts_exterieur: row.get(10)?,
                })
            },
        );

        // On convertit le résultat en Option (Some si trouvé, None si introuvable)
        
        resultat
            .optional()
            .map_err(|e| format!("Erreur SQL sur le match {} : {}", match_id, e))
    }

    

    fn save_resultat_match(&self, resultat: &ResultatSimulationMatch) -> Result<(), String> {
         

       self.conn
            .execute(
                "
                INSERT INTO resultats_matchs (match_id, buts_domicile, buts_exterieur)
                VALUES (?, ?, ?)
                ON CONFLICT(match_id) DO UPDATE SET
                    buts_domicile = excluded.buts_domicile,
                    buts_exterieur = excluded.buts_exterieur
                ",
                params![
                    resultat.match_id,
                    resultat.buts_domicile,
                    resultat.buts_exterieur
                ],
            )
            .map_err(|e| format!("Erreur save_resultat_match : {}", e))?;

        Ok(())
    }
}

