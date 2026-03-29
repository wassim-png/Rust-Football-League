use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Match};
use crate::prochain_match::persistNextGame::sql_next_game_dao::SqlNextGameDAO;
use crate::prochain_match::persistNextGame::next_game_dao::NextGameDAO;

pub struct NextGameManager {
    dao: Box<dyn NextGameDAO>,
}

impl NextGameManager {
    pub fn new(conn: Arc<Connection>, ) -> Self {
        Self {
            dao: Box::new(SqlNextGameDAO { conn }),
        }
    }

    pub fn get_next_game(&self, club_id: i32, journee: i32) -> rusqlite::Result<Match> {
        self.dao.find_next_game_by_club_id(club_id, journee)
    }

}