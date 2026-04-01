use rusqlite::Result;
use crate::models::Match;

pub trait CalendrierDAO {
    fn get_saison_id(&self) -> Result<i32>;
    fn get_club_ids(&self) -> Result<Vec<i32>>;
    fn calendrier_existe(&self, saison_id: i32) -> Result<bool>;
    fn generer_calendrier(&self, saison_id: i32, club_ids: Vec<i32>) -> Result<()>;
    fn get_tous_matchs(&self, saison_id: i32) -> Result<Vec<Match>>;
    fn get_tous_matchs_par_journee(&self, saison_id: i32, journee: i32) -> Result<Vec<Match>>;
}
