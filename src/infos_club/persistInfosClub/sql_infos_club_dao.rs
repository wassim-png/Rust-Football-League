use std::sync::Arc;
use rusqlite::{Connection, Result, Row}; 
use crate::infos_club::persistInfosClub::infos_club_dao::InfosClubDAO;
use crate::models::InfosClub;

pub struct SqliteInfosClubDAO{
    pub conn: Arc<Connection>,
}

impl InfosClubDAO for SqliteInfosClubDAO {
    fn get_infos_by_club(&self, id: i32) -> Result<InfosClub> {
            
            self.conn.query_row("SELECT 
                c.id, 
                c.nom, 
                i.nom_stade, 
                i.stade_capacite, 
                c.reputation, 
                c.avantage_domicile, 
                c.revenu_par_journee_eur, 
                i.url_logo, 
                i.url_stade,
                i.nom_meilleur_buteur
            FROM clubs c
            INNER JOIN info_club i ON c.id = i.club_id 
            WHERE c.id = ?1",
            [id],
             |row: &Row| {
            
                Ok(InfosClub {
                    club_id: row.get(0)?,
                    nom: row.get(1)?,
                    nom_stade: row.get(2)?,
                    stade_capacite: row.get(3)?,
                    reputation: row.get(4)?,
                    avantage_domicile: row.get(5)?,
                    revenu_par_journee_eur: row.get(6)?,
                    url_logo: row.get(7)?,
                    url_stade: row.get(8)?,
                    nom_meilleur_buteur: row.get(9)?
                })
            },
        )
    }
           
}

