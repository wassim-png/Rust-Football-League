use rusqlite::Connection;
use crate::models::Joueur;
use std::sync::Arc;
use crate::composition::persist_joueur::sql_joueur_dao::SqliteJoueurDAO;
use crate::composition::persist_joueur::joueur_dao::JoueurDAO;

#[allow(dead_code)]
pub struct JoueurManager {
    dao: SqliteJoueurDAO,
}

#[allow(dead_code)]
impl JoueurManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: SqliteJoueurDAO { conn },
        }
    }

    pub fn obtenir_joueurs_du_club(&self, club_id: i32) -> rusqlite::Result<Vec<Joueur>> {
        self.dao.get_joueurs_by_club_id(club_id)
    }
}
