use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::Match;
use crate::calendrier::persistCalendrier::sql_calendrier_dao::SqlCalendrierDAO;
use crate::calendrier::persistCalendrier::calendrier_dao::CalendrierDAO;

pub struct CalendrierFacade {
    dao: SqlCalendrierDAO,
    conn: Arc<Connection>,
}

impl CalendrierFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: SqlCalendrierDAO { conn: conn.clone() },
            conn,
        }
    }

    /// Récupère saison_id = Ligue 1 (première saison en DB)
    fn get_saison_id(&self) -> Result<i32> {
        self.conn.query_row(
            "SELECT id FROM saisons LIMIT 1",
            [],
            |row| row.get(0),
        )
    }

    /// Génère le calendrier si absent, puis retourne tous les matchs triés
    pub fn init_et_get_matchs(&self) -> Result<Vec<Match>> {
        let saison_id = self.get_saison_id()?;

        if !self.dao.calendrier_existe(saison_id)? {
            let club_ids: Vec<i32> = {
                let mut stmt = self.conn.prepare("SELECT id FROM clubs ORDER BY id")?;
                let iter = stmt.query_map([], |row| row.get(0))?;
                iter.filter_map(|r| r.ok()).collect()
            };
            self.dao.generer_calendrier(saison_id, club_ids)?;
        }

        self.dao.get_tous_matchs(saison_id)
    }
}
