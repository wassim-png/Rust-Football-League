use rusqlite::Connection;

use std::sync::Arc;
use crate::infos_club::persistInfosClub::sql_infos_club_dao::SqliteInfosClubDAO;
use crate::infos_club::persistInfosClub::infos_club_dao::InfosClubDAO;
use crate::models::InfosClub;

pub struct InfosClubManager {
    
    dao: SqliteInfosClubDAO, 
}

impl InfosClubManager {
    pub fn new(conn:Arc<Connection>) -> Self {
        Self {
           
            dao: SqliteInfosClubDAO { conn },
        }
    }

    pub fn obtenir_infos_club(&self, id: i32) -> rusqlite::Result<InfosClub> {
        
        self.dao.get_infos_by_club(id)
    }
}