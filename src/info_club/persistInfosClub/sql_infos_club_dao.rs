use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::models::Club;
use crate::selection_club::persistClub::club_dao::ClubDAO;

pub struct SqliteInfosClubDAO{
    pub conn: Arc<Connection>,
}

impl InfosClubDAO for SqliteInfosClubDAO {
     fn get_infos_by_club(&self) -> Result<InfosClub> {
            // 1. On prépare la requête SQL
            self.conn.query_row("SELECT club_id, nom, nom_stade, reputation, avantage_domicile, 
            revenu_par_journee, url_logo, url_stade from infos_club FROM club 

            INNER JOIN info_club i ON c.id = i.club_id WHERE c.id = ?",
            [id],
             |row: &Row| {
            
                Ok(InfosClub {
                    club_id: row.get(0)?,
                    nom: row.get(1)?,
                    nom_stade: row.get(2)?,
                    reputation: row.get(3)?,
                    avantage_domicile: row.get(4)?,
                    revenu_par_journee_eur: row.get(5)?,
                    url_logo: row.get(6)?,
                    url_stade: row.get(7)?
                })
            },
        )
    }
           

        
}

