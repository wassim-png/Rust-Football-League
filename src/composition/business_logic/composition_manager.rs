use std::collections::HashSet;
use std::sync::OnceLock;

use crate::simulation::config::composition_rules::CompositionRules;
use crate::models::{CompositionMatch, Joueur};

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

    // =========================
    // NOTE GENERALE
    // =========================
    fn calculer_note_generale(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        let somme: f32 = joueurs
    .iter()
    .filter_map(|j| j.note_actuelle) 
    .map(|note| note as f32)         
    .sum();
        somme / joueurs.len() as f32
    }

    // =========================
    // FORME GENERALE
    // =========================
    fn calculer_forme_generale(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        let somme: f32 = joueurs
            .iter()
            .filter_map(|j| j.forme)     
            .map(|forme| forme as f32)   
            .sum();
        somme / joueurs.len() as f32
    }

    // =========================
    // COLLECTIF
    // =========================
    fn calculer_note_collectif(&self, joueurs: &[Joueur]) -> f32 {
        if joueurs.is_empty() {
            return 0.0;
        }

        let mut nationalites = HashSet::new();
        let mut nb_fr = 0;

        for joueur in joueurs {
            nationalites.insert(joueur.nationalite.clone());

            if joueur.nationalite == Some("France".to_string()) {
                nb_fr += 1;
            }
        }

        let nb_nationalites = nationalites.len() as f32;
        let base = CompositionRules::BASE_COLLECTIF;

        let bonus_cohesion =
            (joueurs.len() as f32 - nb_nationalites) * CompositionRules::BONUS_COHESION_PAR_JOUEUR;

        let bonus_fr = nb_fr as f32 * CompositionRules::BONUS_FRANCAIS;

        let note = base + bonus_cohesion + bonus_fr;

        note.clamp(0.0, CompositionRules::MAX_COLLECTIF)
    }

    // =========================
    // FINITION
    // =========================
    fn calculer_finition(&self, joueurs: &[Joueur]) -> f32 {
        let mut somme = 0.0;
        let mut total_poids = 0.0;

        for joueur in joueurs {
            if let Some(note) = joueur.note_actuelle {
            let poids = match joueur.poste.as_str() {
                "GARDIEN" => CompositionRules::POIDS_GARDIEN,
                "DEFENSE" => CompositionRules::POIDS_DEFENSE,
                "MILIEU" => CompositionRules::POIDS_MILIEU,
                "ATTAQUE" => CompositionRules::POIDS_ATTAQUE,
                _ => 0.0,
            };

            somme += (note as f32) * poids;
            total_poids += poids;
        }}

        if total_poids == 0.0 {
            return 0.0;
        }

        somme / total_poids
    }
}