use rusqlite::Connection;
use crate::models::Joueur;
use std::sync::Arc;
use crate::composition::business_logic::joueur_manager::JoueurManager;

#[allow(dead_code)]
pub struct JoueurFacade {
    manager: JoueurManager,
}

#[allow(dead_code)]
impl JoueurFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: JoueurManager::new(conn),
        }
    }

    pub fn get_joueurs_du_club(&self, club_id: i32) -> rusqlite::Result<Vec<Joueur>> {
        self.manager.obtenir_joueurs_du_club(club_id)
    }
}
