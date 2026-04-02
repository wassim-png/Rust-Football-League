use crate::models::Club;
use rusqlite::Result;

#[allow(dead_code)]
pub trait ClubDAO {
    fn get_all_clubs(&self) -> Result<Vec<Club>>;
    fn get_all_clubs_by_points(&self) -> Result<Vec<Club>>;
    fn get_club_by_id(&self, id: i32) -> Result<Club>;
    fn update_club(&self, club: &Club) -> Result<(), String>;
    fn reset_saison(&self) -> Result<(), String>;
}