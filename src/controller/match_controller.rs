use crate::facade::match_facade::MatchFacade;
use crate::model::resultat_match::ResultatSimulationMatch;

pub struct MatchController;

impl MatchController {
    pub fn new() -> Self {
        Self
    }

    pub fn simuler_match(
        &self,
        match_id: i32,
    ) -> Result<ResultatSimulationMatch, String> {
        MatchFacade::get_instance().simuler_match_et_sauvegarder(match_id)
    }
}