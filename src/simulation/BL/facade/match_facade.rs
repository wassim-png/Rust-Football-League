use std::sync::OnceLock;

use crate::manager::match_manager::MatchManager;
use crate::model::resultat_match::ResultatSimulationMatch;

pub struct MatchFacade;

static INSTANCE: OnceLock<MatchFacade> = OnceLock::new();

impl MatchFacade {
    pub fn get_instance() -> &'static MatchFacade {
        INSTANCE.get_or_init(|| MatchFacade)
    }

    pub fn simuler_match_et_sauvegarder(
        &self,
        match_id: i32,
    ) -> Result<ResultatSimulationMatch, String> {
        MatchManager::get_instance().simuler_match_et_sauvegarder(match_id)
    }
}