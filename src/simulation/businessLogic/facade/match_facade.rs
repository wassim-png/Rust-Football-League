use rusqlite::Connection;
use std::sync::Arc;

use crate::models::{Club, CompositionMatch, ResultatSimulationMatch};
use crate::simulation::businessLogic::manager::match_manager::MatchManager;

pub struct MatchFacade {
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
        equipe_domicile: &mut CompositionMatch,
        equipe_exterieur: &mut CompositionMatch,
        club_domicile: &mut Club,
        club_exterieur: &mut Club,
    ) -> Result<ResultatSimulationMatch, String> {
        self.manager.simuler_match_et_sauvegarder(
            match_id,
            equipe_domicile,
            equipe_exterieur,
            club_domicile,
            club_exterieur,
        )
    }
}