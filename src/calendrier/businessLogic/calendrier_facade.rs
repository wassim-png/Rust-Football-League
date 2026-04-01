use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::Match;
use crate::calendrier::businessLogic::calendrier_manager::CalendrierManager;

pub struct CalendrierFacade {
    manager: CalendrierManager,
}

impl CalendrierFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: CalendrierManager::new(conn),
        }
    }

    pub fn init_et_get_matchs(&self) -> Result<Vec<Match>> {
        self.manager.init_et_get_matchs()
    }

    pub fn get_tous_matchs_par_journee(&self, saison_id: i32, journee: i32)-> Result<Vec<Match>> {
        self.manager.get_tous_matchs_par_journee(saison_id, journee)
    }
}
