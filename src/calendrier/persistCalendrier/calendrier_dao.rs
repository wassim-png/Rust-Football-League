use rusqlite::Result;
use crate::models::Match;

pub trait CalendrierDAO {
    fn calendrier_existe(&self, saison_id: i32) -> Result<bool>;
    fn generer_calendrier(&self, saison_id: i32, club_ids: Vec<i32>) -> Result<()>;
    fn get_tous_matchs(&self, saison_id: i32) -> Result<Vec<Match>>;
}
