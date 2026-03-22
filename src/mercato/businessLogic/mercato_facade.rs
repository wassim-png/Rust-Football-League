use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Joueur, OffreTransfert};
use crate::mercato::persistMercato::mercato_dao::MercatoDAO;
use crate::mercato::persistMercato::sql_mercato_dao::SqlMercatoDAO;

pub struct MercatoFacade {
    dao: SqlMercatoDAO,
}

impl MercatoFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: SqlMercatoDAO { conn },
        }
    }

    pub fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        self.dao.get_tous_joueurs_disponibles(mon_club_id)
    }

    pub fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>> {
        self.dao.generer_offres_ia(mon_club_id)
    }
}
