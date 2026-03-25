use rusqlite::params;

use crate::dao::match_dao::MatchDao;
use crate::db::db_connection::DbConnection;
use crate::model::match::Match;
use crate::model::resultat_match::ResultatSimulationMatch;

pub struct SqliteMatchDao;

impl SqliteMatchDao {
    pub fn new() -> Self {
        Self
    }
}

impl MatchDao for SqliteMatchDao {
    fn find_match_by_id(&self, match_id: i32) -> Result<Option<Match>, String> {
        let connection = DbConnection::get_connection()
            .map_err(|e| format!("Erreur connexion SQLite : {}", e))?;

        let mut stmt = connection
            .prepare(
                "
                SELECT id, saison_id, journee, club_domicile_id, club_exterieur_id, date_coup_envoi
                FROM matchs
                WHERE id = ?
                ",
            )
            .map_err(|e| format!("Erreur préparation find_match_by_id : {}", e))?;

        let mut rows = stmt
            .query(params![match_id])
            .map_err(|e| format!("Erreur exécution find_match_by_id : {}", e))?;

        match rows.next().map_err(|e| format!("Erreur lecture match : {}", e))? {
            Some(row) => Ok(Some(Match {
                id: row.get(0).map_err(|e| format!("Erreur id : {}", e))?,
                saison_id: row.get(1).map_err(|e| format!("Erreur saison_id : {}", e))?,
                journee: row.get(2).map_err(|e| format!("Erreur journee : {}", e))?,
                club_domicile_id: row
                    .get(3)
                    .map_err(|e| format!("Erreur club_domicile_id : {}", e))?,
                club_exterieur_id: row
                    .get(4)
                    .map_err(|e| format!("Erreur club_exterieur_id : {}", e))?,
                date_coup_envoi: row
                    .get(5)
                    .map_err(|e| format!("Erreur date_coup_envoi : {}", e))?,
            })),
            None => Ok(None),
        }
    }

    fn save_resultat_match(&self, resultat: &ResultatSimulationMatch) -> Result<(), String> {
        let connection = DbConnection::get_connection()
            .map_err(|e| format!("Erreur connexion SQLite : {}", e))?;

        connection
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