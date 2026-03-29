use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Joueur, OffreTransfert};
use crate::mercato::persistMercato::mercato_dao::MercatoDAO;
use crate::mercato::persistMercato::sql_mercato_dao::SqlMercatoDAO;

pub struct MercatoManager {
    dao: Box<dyn MercatoDAO>,
}

impl MercatoManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            dao: Box::new(SqlMercatoDAO { conn }),
        }
    }

    pub fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        self.dao.get_tous_joueurs_disponibles(mon_club_id)
    }

    pub fn get_joueurs_mon_club(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        self.dao.get_joueurs_mon_club(mon_club_id)
    }

    pub fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>> {
        self.dao.generer_offres_ia(mon_club_id)
    }

    pub fn recruter_joueur(&self, joueur_id: i32, club_id: i32) -> Result<()> {
        self.dao.recruter_joueur(joueur_id, club_id)
    }

    pub fn vendre_joueur(&self, joueur_id: i32, nouveau_club_id: Option<i32>) -> Result<()> {
        self.dao.vendre_joueur(joueur_id, nouveau_club_id)
    }
}
