use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::prochain_match::persistNextGame::SqliteNextGameDAO;
use crate::models::Match;

pub struct SqlNextGameDAO{
    pub conn: Arc<Connection>,
}

impl NextGameDAO for SqlNextGameDAO {
pub fn find_next_game_by_club_id(&self, id: i32) -> Result<Match> {
        self.conn.query_row(
            "SELECT 
                m.id,
                m.journee,
                m.club_domicile_id,
                c1.nom,
                i1.url_logo,
                m.club_exterieur_id,
                c2.nom,
                i2.url_logo,
                m.date_coup_envoi,
                m.buts_domicile,
                m.buts_exterieur
            FROM matchs m
            INNER JOIN clubs c1 ON m.club_domicile_id = c1.id
            INNER JOIN info_club i1 ON c1.id = i1.club_id
            INNER JOIN clubs c2 ON m.club_exterieur_id = c2.id
            INNER JOIN info_club i2 ON c2.id = i2.club_id
            WHERE (m.club_domicile_id = ?1 OR m.club_exterieur_id = ?1)
            AND m.date_coup_envoi IS NOT NULL
            ORDER BY m.date_coup_envoi ASC
            LIMIT 1",
            params![id],
            |row: &Row| {
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
        )
    }
}
           
