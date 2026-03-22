use rusqlite::Connection;
use crate::models::Club;
use std::sync::Arc;
use crate::selection_club::persistClub::sql_club_dao::SqliteClubDAO;
use crate::selection_club::persistClub::club_dao::ClubDAO; 

pub struct ClubManager {
    
    dao: SqliteClubDAO, 
}

impl ClubManager {
    pub fn new(conn:Arc<Connection>) -> Self {
        Self {
           
            dao: SqliteClubDAO { conn },
        }
    }

    pub fn obtenir_tous_les_clubs(&self) -> rusqlite::Result<Vec<Club>> {
        
        self.dao.get_all_clubs()
    }
}