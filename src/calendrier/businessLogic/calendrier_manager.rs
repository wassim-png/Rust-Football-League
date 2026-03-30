use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::Match;
use crate::calendrier::persistCalendrier::calendrier_dao::CalendrierDAO;
use crate::calendrier::persistCalendrier::sql_calendrier_dao::SqlCalendrierDAO;

pub struct CalendrierManager {
    dao: Box<dyn CalendrierDAO>,
}

impl CalendrierManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: Box::new(SqlCalendrierDAO { conn }),
        }
    }

    /// Génère le calendrier si absent, puis retourne tous les matchs triés
    pub fn init_et_get_matchs(&self) -> Result<Vec<Match>> {
        let saison_id = self.dao.get_saison_id()?;

        if !self.dao.calendrier_existe(saison_id)? {
            let club_ids = self.dao.get_club_ids()?;
            self.dao.generer_calendrier(saison_id, club_ids)?;
        }

        self.dao.get_tous_matchs(saison_id)
    }

    pub fn get_tous_matchs_par_journee(&self, saison_id: i32, journee: i32) -> Result<Vec<Match>>{
        self.dao.get_tous_matchs_par_journee(saison_id, journee)
    }
}
