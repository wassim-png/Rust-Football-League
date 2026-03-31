use rusqlite::Connection;
use std::sync::Arc;

use crate::composition::business_logic::composition_manager::CompositionManager;
use crate::models::{CompositionMatch, Joueur};

pub struct CompositionFacade {
    manager: CompositionManager,
}

impl CompositionFacade {
    pub fn new(_conn: Arc<Connection>) -> Self {
        Self {
            manager: CompositionManager::new(),
        }
    }

    pub fn creer_composition_match(
        &self,
        match_id: i32,
        club_id: i32,
        joueurs: &[Joueur],
    ) -> CompositionMatch {
        self.manager
            .creer_composition_match(match_id, club_id, joueurs)
    }
}