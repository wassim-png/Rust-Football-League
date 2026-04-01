use crate::models::Match;
use rusqlite::Result;

pub trait NextGameDAO {
    fn find_next_game_by_club_id(&self, id: i32, journee:i32) -> Result<Match>;
    
}