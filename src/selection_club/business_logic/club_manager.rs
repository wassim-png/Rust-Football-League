use rusqlite::Connection;
use std::sync::Arc;

use crate::models::Club;
use crate::selection_club::persist_club::club_dao::ClubDAO;
use crate::selection_club::persist_club::sql_club_dao::SqlClubDAO;

pub struct ClubManager {
    dao: Box<dyn ClubDAO>,
}

impl ClubManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: Box::new(SqlClubDAO { conn }),
        }
    }

    pub fn obtenir_tous_les_clubs(&self) -> rusqlite::Result<Vec<Club>> {
        self.dao.get_all_clubs()
    }

    pub fn obtenir_tous_les_clubs_par_points(&self) ->rusqlite::Result<Vec<Club>> {
        self.dao.get_all_clubs_by_points()
    }

    
}