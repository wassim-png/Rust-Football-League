use rusqlite::Connection;
use crate::models::Club;
use std::sync::Arc;
use crate::selection_club::businessLogic::club_manager::ClubManager;

pub struct ClubFacade {
    manager: ClubManager,
}

impl ClubFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: ClubManager::new(conn),
        }
    }

  
}