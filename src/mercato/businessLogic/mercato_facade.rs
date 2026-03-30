use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{Joueur, OffreTransfert};
use crate::mercato::businessLogic::mercato_manager::MercatoManager;

pub struct MercatoFacade {
    manager: MercatoManager,
}

impl MercatoFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: MercatoManager::new(conn),
        }
    }

    pub fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        self.manager.get_tous_joueurs_disponibles(mon_club_id)
    }

    pub fn get_joueurs_mon_club(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        self.manager.get_joueurs_mon_club(mon_club_id)
    }

    pub fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>> {
        self.manager.generer_offres_ia(mon_club_id)
    }

    pub fn recruter_joueur(&self, joueur_id: i32, club_id: i32) -> Result<()> {
        self.manager.recruter_joueur(joueur_id, club_id)
    }

    pub fn vendre_joueur(&self, joueur_id: i32, nouveau_club_id: Option<i32>) -> Result<()> {
        self.manager.vendre_joueur(joueur_id, nouveau_club_id)
    }
}
