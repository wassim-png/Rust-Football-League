use rusqlite::Connection;
use crate::models::Club;
use std::sync::Arc;
use crate::selection_club::business_logic::club_manager::ClubManager;

pub struct ClubFacade {
    manager: ClubManager,
}

impl ClubFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: ClubManager::new(conn),
        }
    }

    pub fn get_all(&self) -> rusqlite::Result<Vec<Club>> {
        self.manager.obtenir_tous_les_clubs()
    }

    pub fn get_all_clubs_by_points(&self) -> rusqlite::Result<Vec<Club>> {
        self.manager.obtenir_tous_les_clubs_par_points()
    }
}