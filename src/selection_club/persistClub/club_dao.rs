use crate::models::Club;
use rusqlite::Result;

pub trait ClubDAO {
    fn get_all_clubs(&self) -> Result<Vec<Club>>;
    fn get_club_by_id(&self, id: i32) -> Result<Club>;
}