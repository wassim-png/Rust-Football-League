use rusqlite::Connection;
use std::sync::Arc;
 use crate::infos_club::businessLogic::infos_club_manager::InfosClubManager;
 use crate::models::InfosClub;

pub struct InfosClubFacade {
    manager: InfosClubManager,
}

impl InfosClubFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: InfosClubManager::new(conn),
        }
    }

     pub fn obtenir_infos_club(&self, id: i32) -> rusqlite::Result<InfosClub> {
        
        self.manager.obtenir_infos_club(id)
    }

  
}