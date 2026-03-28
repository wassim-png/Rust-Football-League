use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Match};
use crate::prochain_match::businessLogic::NextGameManager;


pub struct NextGameFacade {
    manager: NextGameManager,
}

impl NextGameFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: NextGameManager::new(conn),
        }
    }
    pub fn get_next_game(&self, mon_club_id: i32) -> rusqlite::Result<Vec<Joueur>> {
        self.manager.get_next_game(mon_club_id)
    }

}

