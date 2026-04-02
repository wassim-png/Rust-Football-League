use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::{Club, CompositionMatch, Joueur, Match, ResultatMatchJournee};
use crate::simulation::businessLogic::manager::match_manager::MatchManager;

pub struct MatchFacade {
    manager: MatchManager,
}

impl MatchFacade {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            manager: MatchManager::new(conn),
        }
    }

    pub fn simuler_journee(
        &self,
        matchs: &[Match],
        club_utilisateur_id: i32,
        composition_utilisateur: &CompositionMatch,
        clubs: &[Club],
        joueurs_par_club: &HashMap<i32, Vec<Joueur>>,
    ) -> Result<Vec<ResultatMatchJournee>, String> {
        self.manager.simuler_journee(
            matchs,
            club_utilisateur_id,
            composition_utilisateur,
            clubs,
            joueurs_par_club,
        )
    }

    pub fn appliquer_recuperation_forme_globale(&self, joueurs_exclus: &[i32]) {
        self.manager.appliquer_recuperation_forme_globale(joueurs_exclus);
    }
}