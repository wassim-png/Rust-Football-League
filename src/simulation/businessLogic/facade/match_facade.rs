use std::sync::OnceLock;
use std::sync::Arc;
use rusqlite::Connection;
use crate::simulation::businessLogic::manager::match_manager::MatchManager;
use crate::models::ResultatSimulationMatch;

pub struct MatchFacade
    {
    manager: MatchManager,
}




impl MatchFacade {
     pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: MatchManager::new(conn),
        }
    }

    pub fn simuler_match_et_sauvegarder(
        &self,
        match_id: i32,
    ) -> Result<ResultatSimulationMatch, String> {
       self.manager.simuler_match_et_sauvegarder(match_id)
    }
}