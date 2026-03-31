use std::sync::Arc;
use rusqlite::Connection;
use crate::models::{Match};
use crate::prochain_match::businessLogic::next_game_manager::NextGameManager;


pub struct NextGameFacade {
    manager: NextGameManager,
}

impl NextGameFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: NextGameManager::new(conn),
        }
    }
    pub fn get_next_game(&self, mon_club_id: i32, journee: i32) -> rusqlite::Result<Match> {
        self.manager.get_next_game(mon_club_id, journee)
    }

}

