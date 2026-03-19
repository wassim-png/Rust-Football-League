use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::models::Club;
use crate::selection_club::persistClub::club_dao::ClubDAO;

pub struct SqliteClubDAO{
    pub conn: Arc<Connection>,
}

impl ClubDAO for SqliteClubDAO {

    fn get_all_clubs(&self) -> Result<Vec<Club>> {
            // 1. On prépare la requête SQL
            let mut stmt = self.conn.prepare("SELECT id, nom, nom_court, reputation, budget_eur, revenu_par_journee_eur, avantage_domicile FROM clubs"
    )?;

            // 2. On mappe chaque ligne (row) vers une instance de la struct Club
            let club_iter = stmt.query_map([], |row: &Row| {
                Ok(Club {
                    id: row.get(0)?,
                    nom: row.get(1)?,
                    nom_court: row.get(2)?,
                    reputation: row.get(3)?,
                    budget_eur: row.get(4)?,
                    revenu_par_journee_eur: row.get(5)?,
                    avantage_domicile: row.get(6)?
                })
            })?;

            // 3. On transforme l'itérateur en un Vecteur de résultats
            let mut clubs = Vec::new();
            for club in club_iter {
                clubs.push(club?);
            }

            Ok(clubs)
        }

         fn get_club_by_id(&self, id: i32) -> rusqlite::Result<Club> {
        self.conn.query_row(
            "SELECT id, nom, nom_court, reputation, budget_eur, revenu_par_journee, avantage_domicile
             FROM clubs WHERE id = ?",
            [id], 
            |row: &Row| {
            
                Ok(Club {
                    id: row.get(0)?,
                    nom: row.get(1)?,
                    nom_court: row.get(2)?,
                    reputation: row.get(3)?,
                    budget_eur: row.get(4)?,
                    revenu_par_journee_eur: row.get(5)?,
                    avantage_domicile: row.get(6)?
                })
            },
        )
    }

}

