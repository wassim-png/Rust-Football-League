use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::{ErreurMercato, Joueur, OffreTransfert};
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

    /// Recrute un joueur libre au prix fixe de sa valeur marchande.
    pub fn recruter_joueur_libre(
        &self,
        joueur: &Joueur,
        club_id: i32,
        budget_actuel: i64,
    ) -> Result<i64, ErreurMercato> {
        let cout = joueur.valeur_marche_eur;
        if budget_actuel < cout {
            return Err(ErreurMercato::BudgetInsuffisant { budget: budget_actuel, cout });
        }
        self.dao.recruter_joueur(joueur.id, club_id)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        let nouveau_budget = budget_actuel - cout;
        self.dao.mettre_a_jour_budget_club(club_id, nouveau_budget)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        Ok(nouveau_budget)
    }

    /// Soumet une offre pour un joueur sous contrat.
    /// Acceptation selon seuil basé sur la réputation du joueur.
    pub fn faire_offre_transfert(
        &self,
        joueur: &Joueur,
        montant: i64,
        club_id: i32,
        budget_actuel: i64,
    ) -> Result<i64, ErreurMercato> {
        if budget_actuel < montant {
            return Err(ErreurMercato::BudgetInsuffisant { budget: budget_actuel, cout: montant });
        }
        let seuil = if joueur.reputation > 90 { 1.15 }
                    else if joueur.reputation > 80 { 1.0 }
                    else { 0.85 };
        let seuil_montant = (joueur.valeur_marche_eur as f64 * seuil) as i64;
        if montant < seuil_montant {
            return Err(ErreurMercato::OffreRefusee {
                club: joueur.club_nom.clone().unwrap_or_default(),
                montant,
            });
        }
        self.dao.recruter_joueur(joueur.id, club_id)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        let nouveau_budget = budget_actuel - montant;
        self.dao.mettre_a_jour_budget_club(club_id, nouveau_budget)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        Ok(nouveau_budget)
    }

    /// Accepte une offre reçue d'un club adverse.
    pub fn accepter_offre_recue(
        &self,
        joueur_id: i32,
        club_acheteur_id: i32,
        montant: i64,
        club_vendeur_id: i32,
        budget_actuel: i64,
        taille_effectif: usize,
    ) -> Result<i64, ErreurMercato> {
        if taille_effectif <= 15 {
            return Err(ErreurMercato::EffectifMinimum { taille: taille_effectif });
        }
        self.dao.vendre_joueur(joueur_id, Some(club_acheteur_id))
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        let nouveau_budget = budget_actuel + montant;
        self.dao.mettre_a_jour_budget_club(club_vendeur_id, nouveau_budget)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        Ok(nouveau_budget)
    }

    /// Libère un joueur sur le marché libre et récupère sa valeur marchande.
    pub fn vendre_joueur_marche(
        &self,
        joueur: &Joueur,
        club_id: i32,
        budget_actuel: i64,
        taille_effectif: usize,
    ) -> Result<i64, ErreurMercato> {
        if taille_effectif <= 15 {
            return Err(ErreurMercato::EffectifMinimum { taille: taille_effectif });
        }
        self.dao.vendre_joueur(joueur.id, None)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        let nouveau_budget = budget_actuel + joueur.valeur_marche_eur;
        self.dao.mettre_a_jour_budget_club(club_id, nouveau_budget)
            .map_err(|e| ErreurMercato::ErreurDB(e.to_string()))?;
        Ok(nouveau_budget)
    }
}
