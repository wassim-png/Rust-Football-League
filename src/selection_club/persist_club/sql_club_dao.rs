use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::models::Club;
use crate::selection_club::persist_club::club_dao::ClubDAO;

pub struct SqlClubDAO{
    pub conn: Arc<Connection>,
}

impl ClubDAO for SqlClubDAO {

    fn get_all_clubs(&self) -> Result<Vec<Club>> {
            // 1. On prépare la requête SQL
            let mut stmt = self.conn.prepare("SELECT id, nom, nom_court, reputation, budget_eur, revenu_par_journee_eur, 
            avantage_domicile, url_logo, 
            points, buts_marques, buts_encaisses FROM clubs c
            INNER JOIN info_club i on i.club_id = c.id ORDER BY nom ASC"
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
                    avantage_domicile: row.get(6)?,
                    url_logo: row.get(7)?,
                    points: row.get(8)?,
                    buts_marques: row.get(9)?,
                    buts_encaisses: row.get(10)?
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
            "SELECT id, nom, nom_court, reputation, budget_eur, revenu_par_journee, avantage_domicile,
            points, buts_marques, buts_encaisse
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
                    avantage_domicile: row.get(6)?,
                    url_logo: row.get(7)?,
                    points: row.get(8)?,
                    buts_marques: row.get(9)?,
                    buts_encaisses: row.get(10)?
                })
            },
        )
    }

    fn update_club(&self, club: &Club) -> Result<(), String> {
        let id = club.id.expect("Erreur : Impossible de mettre à jour un club sans ID !");

       
        self.conn.execute(
            "UPDATE clubs SET 
                nom = ?1,
                nom_court = ?2,
                reputation = ?3,
                budget_eur = ?4,
                revenu_par_journee_eur = ?5,
                avantage_domicile = ?6,
                url_logo = ?7,
                points = ?8,
                buts_marques = ?9,
                buts_encaisses = ?10
            WHERE id = ?11",
            rusqlite::params![
                club.nom,
                club.nom_court,
                club.reputation,
                club.budget_eur,
                club.revenu_par_journee_eur,
                club.avantage_domicile,
                club.url_logo,
                club.points,
                club.buts_marques,
                club.buts_encaisses,
                id
            ],
        ).map_err(|e| format!("Erreur lors de la mise à jour du club : {}", e))?; 
        Ok(())
    }

}

