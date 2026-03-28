use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Match};
use crate::persistNextGame::sql_next_game_dao::SqlNextGameDAO

pub struct NextGameManager {
    dao: SqlNextGameDAO,
}

impl NextGameManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: SqlNextGameDAO { conn },
        }
    }

    pub fn get_next_game(&self, club_id: i32) -> rusqlite::Result<Match> {
        self.dao.find_next_game_by_clubId(club_id)
    }

}