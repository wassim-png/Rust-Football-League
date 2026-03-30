use crate::models::Match;
use crate::models::ResultatSimulationMatch;

pub trait MatchDao {
    fn find_match_by_id(&self, match_id: i32) -> Result<Option<Match>, String>;
    fn save_resultat_match(&self, resultat: &ResultatSimulationMatch) -> Result<(), String>;
}