use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::prochain_match::persistNextGame::next_game_dao::NextGameDAO;
use crate::models::Match;

pub struct SqlNextGameDAO{
    pub conn: Arc<Connection>,
}

impl NextGameDAO for SqlNextGameDAO {
fn find_next_game_by_club_id(&self, id: i32, journee:i32) -> Result<Match> {
        self.conn.query_row(
            "SELECT m.id, m.journee,
                    m.club_domicile_id,  cd.nom,  id_info.url_logo,
                    m.club_exterieur_id, ce.nom,  ie_info.url_logo,
                    m.date_coup_envoi,
                    r.buts_domicile, r.buts_exterieur
                FROM matchs m
                LEFT JOIN clubs cd       ON cd.id       = m.club_domicile_id
                LEFT JOIN clubs ce       ON ce.id       = m.club_exterieur_id
                LEFT JOIN info_club id_info ON id_info.club_id = m.club_domicile_id
                LEFT JOIN info_club ie_info ON ie_info.club_id = m.club_exterieur_id
                LEFT JOIN resultats_matchs r ON r.match_id = m.id
                WHERE m.journee = ?1
                AND (m.club_domicile_id = ?2 OR m.club_exterieur_id = ?2)
                AND m.date_coup_envoi IS NOT NULL",
            [journee, id],
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
           
