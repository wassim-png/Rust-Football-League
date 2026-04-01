use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{ErreurMercato, Joueur, OffreTransfert};
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

    pub fn recruter_joueur_libre(
        &self,
        joueur: &Joueur,
        club_id: i32,
        budget_actuel: i64,
    ) -> Result<i64, ErreurMercato> {
        self.manager.recruter_joueur_libre(joueur, club_id, budget_actuel)
    }

    pub fn faire_offre_transfert(
        &self,
        joueur: &Joueur,
        montant: i64,
        club_id: i32,
        budget_actuel: i64,
    ) -> Result<i64, ErreurMercato> {
        self.manager.faire_offre_transfert(joueur, montant, club_id, budget_actuel)
    }

    pub fn accepter_offre_recue(
        &self,
        joueur_id: i32,
        club_acheteur_id: i32,
        montant: i64,
        club_vendeur_id: i32,
        budget_actuel: i64,
        taille_effectif: usize,
    ) -> Result<i64, ErreurMercato> {
        self.manager.accepter_offre_recue(
            joueur_id, club_acheteur_id, montant,
            club_vendeur_id, budget_actuel, taille_effectif,
        )
    }

    pub fn vendre_joueur_marche(
        &self,
        joueur: &Joueur,
        club_id: i32,
        budget_actuel: i64,
        taille_effectif: usize,
    ) -> Result<i64, ErreurMercato> {
        self.manager.vendre_joueur_marche(joueur, club_id, budget_actuel, taille_effectif)
    }
}
