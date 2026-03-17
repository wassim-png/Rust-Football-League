use std::collections::HashSet;
use std::sync::OnceLock;

use crate::config::composition_rules::CompositionRules;
use crate::model::composition_match::CompositionMatch;
use crate::model::joueur_composition::JoueurComposition;
use crate::model::poste::Poste;
use crate::model::slot_composition::SlotComposition;

pub struct CompositionManager;

static INSTANCE: OnceLock<CompositionManager> = OnceLock::new();

impl CompositionManager {
    pub fn get_instance() -> &'static CompositionManager {
        INSTANCE.get_or_init(|| CompositionManager)
    }

    // =========================
    // CREATION 4-3-3
    // =========================
    pub fn creer_composition_vide(&self, match_id: i32, club_id: i32) -> CompositionMatch {
        let mut slots = Vec::new();

        // Gardien
        slots.push(self.creer_slot("GK", Poste::Gardien));

        // Défenseurs
        for i in 1..=4 {
            slots.push(self.creer_slot(&format!("DEF{}", i), Poste::Defense));
        }

        // Milieux
        for i in 1..=3 {
            slots.push(self.creer_slot(&format!("MID{}", i), Poste::Milieu));
        }

        // Attaquants
        for i in 1..=3 {
            slots.push(self.creer_slot(&format!("ATT{}", i), Poste::Attaque));
        }

        CompositionMatch {
            match_id,
            club_id,
            slots,
            note_generale: 0.0,
            note_collectif: 0.0,
            forme_generale: 0.0,
            finition: 0.0,
        }
    }

    fn creer_slot(&self, cle: &str, poste: Poste) -> SlotComposition {
        SlotComposition {
            cle: cle.to_string(),
            poste_attendu: poste,
            joueur_selectionne: None,
        }
    }

    // =========================
    // SELECTION JOUEUR
    // =========================
    pub fn selectionner_joueur(
        &self,
        composition: &mut CompositionMatch,
        slot_key: &str,
        joueur: JoueurComposition,
    ) -> Result<(), String> {
        // règle : forme > 0
        if joueur.forme <= 0.0 {
            return Err("Joueur indisponible (forme = 0)".to_string());
        }

        // règle : pas déjà sélectionné
        for slot in &composition.slots {
            if let Some(j) = &slot.joueur_selectionne {
                if j.joueur_id == joueur.joueur_id {
                    return Err("Joueur déjà sélectionné".to_string());
                }
            }
        }

        let slot = composition
            .slots
            .iter_mut()
            .find(|s| s.cle == slot_key)
            .ok_or("Slot introuvable")?;

        slot.joueur_selectionne = Some(joueur);

        self.mettre_a_jour_stats(composition);

        Ok(())
    }

    // =========================
    // CALCUL GLOBAL
    // =========================
    fn mettre_a_jour_stats(&self, composition: &mut CompositionMatch) {
        composition.note_generale = self.calculer_note_generale(&composition.slots);
        composition.note_collectif = self.calculer_note_collectif(&composition.slots);
        composition.forme_generale = self.calculer_forme_generale(&composition.slots);
        composition.finition = self.calculer_finition(&composition.slots);
    }

    // =========================
    // NOTE GENERALE
    // =========================
    fn calculer_note_generale(&self, slots: &[SlotComposition]) -> f32 {
        let mut somme = 0.0;
        let mut count = 0;

        for slot in slots {
            if let Some(joueur) = &slot.joueur_selectionne {
                let note = self.note_joueur(joueur, &slot.poste_attendu);
                somme += note;
                count += 1;
            }
        }

        if count == 0 {
            return 0.0;
        }

        somme / count as f32
    }

    fn note_joueur(&self, joueur: &JoueurComposition, poste_attendu: &Poste) -> f32 {
        if &joueur.poste_reel == poste_attendu {
            joueur.note_generale
        } else {
            joueur.note_generale * (1.0 - CompositionRules::MALUS_HORS_POSTE)
        }
    }

    // =========================
    // FORME GENERALE
    // =========================
    fn calculer_forme_generale(&self, slots: &[SlotComposition]) -> f32 {
        let mut somme = 0.0;
        let mut count = 0;

        for slot in slots {
            if let Some(joueur) = &slot.joueur_selectionne {
                somme += joueur.forme;
                count += 1;
            }
        }

        if count == 0 {
            return 0.0;
        }

        somme / count as f32
    }

    // =========================
    // COLLECTIF
    // =========================
    fn calculer_note_collectif(&self, slots: &[SlotComposition]) -> f32 {
        let joueurs: Vec<_> = slots
            .iter()
            .filter_map(|s| s.joueur_selectionne.as_ref())
            .collect();

        if joueurs.is_empty() {
            return 0.0;
        }

        let mut nationalites = HashSet::new();
        let mut nb_fr = 0;

        for j in &joueurs {
            nationalites.insert(j.nationalite.clone());

            if j.nationalite == "FR" {
                nb_fr += 1;
            }
        }

        let nb_nationalites = nationalites.len() as f32;

        let base = CompositionRules::BASE_COLLECTIF;

        let bonus_cohesion =
            (joueurs.len() as f32 - nb_nationalites)
                * CompositionRules::BONUS_COHESION_PAR_JOUEUR;

        let bonus_fr = nb_fr as f32 * CompositionRules::BONUS_FRANCAIS;

        let note = base + bonus_cohesion + bonus_fr;

        note.clamp(0.0, CompositionRules::MAX_COLLECTIF)
    }

    // =========================
    // FINITION (pondérée)
    // =========================
    fn calculer_finition(&self, slots: &[SlotComposition]) -> f32 {
        let mut somme = 0.0;
        let mut total_poids = 0.0;

        for slot in slots {
            if let Some(joueur) = &slot.joueur_selectionne {
                let poids = match slot.poste_attendu {
                    Poste::Gardien => CompositionRules::POIDS_GARDIEN,
                    Poste::Defense => CompositionRules::POIDS_DEFENSE,
                    Poste::Milieu => CompositionRules::POIDS_MILIEU,
                    Poste::Attaque => CompositionRules::POIDS_ATTAQUE,
                };

                somme += joueur.note_generale * poids;
                total_poids += poids;
            }
        }

        if total_poids == 0.0 {
            return 0.0;
        }

        somme / total_poids
    }

    // =========================
    // VALIDATION
    // =========================
    pub fn composition_complete(&self, composition: &CompositionMatch) -> bool {
        composition
            .slots
            .iter()
            .all(|s| s.joueur_selectionne.is_some())
    }
}