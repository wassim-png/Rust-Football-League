use std::collections::HashSet;
use std::sync::OnceLock;

use crate::config::composition_rules::CompositionRules;
use crate::model::composition_match::CompositionMatch;
use crate::models::Joueur;

pub struct CompositionManager;

static INSTANCE: OnceLock<CompositionManager> = OnceLock::new();

impl CompositionManager {
    pub fn get_instance() -> &'static CompositionManager {
        INSTANCE.get_or_init(|| CompositionManager)
    }

    pub fn creer_composition_match(
        &self,
        match_id: i32,
        club_id: i32,
        joueurs: &[Joueur],
    ) -> CompositionMatch {
        CompositionMatch {
            match_id,
            club_id,
            note_generale: self.calculer_note_generale(joueurs),
            note_collectif: self.calculer_note_collectif(joueurs),
            forme_generale: self.calculer_forme_generale(joueurs),
            finition: self.calculer_finition(joueurs),
        }
    }

    fn calculer_note_generale(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        let somme: f32 = joueurs.iter().map(|j| j.reputation as f32).sum();
        somme / joueurs.len() as f32
    }

    fn calculer_forme_generale(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        // temporaire : forme = réputation
        let somme: f32 = joueurs.iter().map(|j| j.reputation as f32).sum();
        somme / joueurs.len() as f32
    }

    fn calculer_note_collectif(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        // ⚠️ temporaire si tu n'as pas nationalité
        let base = CompositionRules::BASE_COLLECTIF;
        let bonus = joueurs.len() as f32 * 2.0;

        (base + bonus).clamp(0.0, CompositionRules::MAX_COLLECTIF)
    }

    fn calculer_finition(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        let mut somme = 0.0;
        let mut total_poids = 0.0;

        for joueur in joueurs {
            let poids = match joueur.poste.as_str() {
                "GARDIEN" => CompositionRules::POIDS_GARDIEN,
                "DEFENSE" => CompositionRules::POIDS_DEFENSE,
                "MILIEU" => CompositionRules::POIDS_MILIEU,
                "ATTAQUE" => CompositionRules::POIDS_ATTAQUE,
                _ => 1.0,
            };

            somme += joueur.reputation as f32 * poids;
            total_poids += poids;
        }

        if total_poids == 0.0 {
            return 0.0;
        }

        somme / total_poids
    }
}