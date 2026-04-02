use rand::Rng;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;

use crate::composition::business_logic::composition_manager::CompositionManager;
use crate::models::{
    Club, CompositionMatch, Joueur, Match, ResultatMatchJournee, ResultatSimulationMatch,
};
use crate::selection_club::persist_club::club_dao::ClubDAO;
use crate::selection_club::persist_club::sql_club_dao::SqlClubDAO;
use crate::simulation::config::match_rules::MatchRules;
use crate::simulation::persistSimulation::dao::match_dao::MatchDao;
use crate::composition::persist_joueur::joueur_dao::JoueurDAO;
use crate::composition::persist_joueur::sql_joueur_dao::SqliteJoueurDAO ;
use crate::simulation::persistSimulation::sqlitedao::sqlite_match_dao::SqliteMatchDao;

pub struct MatchManager {
    match_dao: Box<dyn MatchDao>,
    club_dao: Box<dyn ClubDAO>,
    joueur_dao: Box<dyn JoueurDAO>
}

impl MatchManager {
    pub fn new(conn: Arc<Connection>) -> Self {
        Self {
            match_dao: Box::new(SqliteMatchDao { conn: conn.clone() }),
            club_dao: Box::new(SqlClubDAO { conn: conn.clone() }),
            joueur_dao: Box::new(SqliteJoueurDAO{conn})
        }
    }

    pub fn calculer_note_globale(&self, equipe: &CompositionMatch) -> f32 {
        MatchRules::COEF_FORME_GENERALE * equipe.forme_generale
            + MatchRules::COEF_NOTE_GENERALE * equipe.note_generale
            + MatchRules::COEF_NOTE_COLLECTIF * equipe.note_collectif
    }

    pub fn calcul_cote_match(
        &self,
        equipe1: &CompositionMatch,
        equipe2: &CompositionMatch,
    ) -> (f32, f32) {
        let res = self.calculer_note_globale(equipe1) - self.calculer_note_globale(equipe2);

        let coef_eq1 = (50.0 + res * MatchRules::COEF_REGULATEUR).clamp(5.0, 95.0);
        let coef_eq2 = 100.0 - coef_eq1;

        (coef_eq1, coef_eq2)
    }

    pub fn calcul_occasions(&self, cote: f32) -> i32 {
        if cote < 40.0 {
            3
        } else if cote < 50.0 {
            4
        } else if cote < 60.0 {
            5
        } else {
            6
        }
    }

    pub fn calcul_proba_conversion(&self, finition: f32) -> f32 {
        (finition * MatchRules::COEF_REGULATEUR_BUT).clamp(5.0, 90.0)
    }

    pub fn simuler_score(
        &self,
        equipe_domicile: &CompositionMatch,
        equipe_exterieur: &CompositionMatch,
    ) -> (i32, i32) {
        let (cote_dom, cote_ext) = self.calcul_cote_match(equipe_domicile, equipe_exterieur);

        let occasions_dom = self.calcul_occasions(cote_dom);
        let occasions_ext = self.calcul_occasions(cote_ext);

        let proba_dom = self.calcul_proba_conversion(equipe_domicile.finition);
        let proba_ext = self.calcul_proba_conversion(equipe_exterieur.finition);

        let mut buts_dom = 0;
        let mut buts_ext = 0;

        let mut rng = rand::thread_rng();

        for _ in 0..occasions_dom {
            let x = rng.gen_range(0.0..100.0);
            if x < proba_dom {
                buts_dom += 1;
            }
        }

        for _ in 0..occasions_ext {
            let x = rng.gen_range(0.0..100.0);
            if x < proba_ext {
                buts_ext += 1;
            }
        }

        (buts_dom, buts_ext)
    }

    fn mettre_a_jour_stats_clubs(
        &self,
        club_domicile: &mut Club,
        club_exterieur: &mut Club,
        buts_dom: i32,
        buts_ext: i32,
    ) {
        club_domicile.buts_marques += buts_dom;
        club_domicile.buts_encaisses += buts_ext;

        club_exterieur.buts_marques += buts_ext;
        club_exterieur.buts_encaisses += buts_dom;

        if buts_dom > buts_ext {
            club_domicile.points += MatchRules::POINTS_VICTOIRE;
            club_exterieur.points += MatchRules::POINTS_DEFAITE;
        } else if buts_ext > buts_dom {
            club_exterieur.points += MatchRules::POINTS_VICTOIRE;
            club_domicile.points += MatchRules::POINTS_DEFAITE;
        } else {
            club_domicile.points += MatchRules::POINTS_NUL;
            club_exterieur.points += MatchRules::POINTS_NUL;
        }
    }

    fn appliquer_baisse_forme_apres_match(&self, composition: &mut CompositionMatch) {
        for joueur in &mut composition.joueurs {
            let perte = match joueur.poste.as_str() {
                "GARDIEN" => MatchRules::PERTE_FORME_GARDIEN,
                "DEFENSE" => MatchRules::PERTE_FORME_DEFENSE,
                "MILIEU" => MatchRules::PERTE_FORME_MILIEU,
                "ATTAQUE" => MatchRules::PERTE_FORME_ATTAQUE,
                _ => 0.0,
            };
            if let Some(forme_actuelle) = joueur.forme {
                let nouvelle_forme_f32 = (forme_actuelle as f32) - perte;
                let forme = (nouvelle_forme_f32 as i32).max(MatchRules::FORME_MIN as i32);
                let _ = self.joueur_dao.update_forme_joueur(joueur.id, forme);
            }
        }
    }
    pub fn appliquer_recuperation_forme_globale(&self, joueurs_exclus: &[i32]) {
        if let Err(e) = self.joueur_dao.recuperation_forme_globale(joueurs_exclus) {
            println!("Erreur lors de la recupération de la forme globale: {}", e);
        } else {
            println!("Forme globale restaurée (+15) pour les joueurs au repos ({} joueurs exclus) !", joueurs_exclus.len());
        }
    }

    pub fn simuler_match(
        &self,
        match_id: i32,
        equipe_domicile: &mut CompositionMatch,
        equipe_exterieur: &mut CompositionMatch,
        club_domicile: &mut Club,
        club_exterieur: &mut Club,
    ) -> ResultatSimulationMatch {
        let (buts_dom, buts_ext) = self.simuler_score(equipe_domicile, equipe_exterieur);

        let vainqueur_id = if buts_dom > buts_ext {
            Some(equipe_domicile.club_id)
        } else if buts_ext > buts_dom {
            Some(equipe_exterieur.club_id)
        } else {
            None
        };

        self.mettre_a_jour_stats_clubs(club_domicile, club_exterieur, buts_dom, buts_ext);
        self.appliquer_baisse_forme_apres_match(equipe_domicile);
        self.appliquer_baisse_forme_apres_match(equipe_exterieur);

        ResultatSimulationMatch {
            match_id,
            buts_domicile: buts_dom,
            buts_exterieur: buts_ext,
            vainqueur_id,
        }
    }

    pub fn simuler_match_et_sauvegarder(
        &self,
        match_id: i32,
        equipe_domicile: &mut CompositionMatch,
        equipe_exterieur: &mut CompositionMatch,
        club_domicile: &mut Club,
        club_exterieur: &mut Club,
    ) -> Result<ResultatSimulationMatch, String> {
        let resultat = self.simuler_match(
            match_id,
            equipe_domicile,
            equipe_exterieur,
            club_domicile,
            club_exterieur,
        );

        self.match_dao.save_resultat_match(&resultat)?;
        self.club_dao.update_club(club_domicile)?;
        self.club_dao.update_club(club_exterieur)?;

        Ok(resultat)
    }

    
    fn choisir_11_meilleurs(&self, joueurs: &[Joueur]) -> Vec<Joueur> {
        let mut joueurs_tries = joueurs.to_vec();

        joueurs_tries.sort_by(|a, b| {
           
            let note_a = a.note_actuelle.unwrap_or(0) as f32;
            let forme_a = a.forme.unwrap_or(0) as f32;
          
            let score_a = (note_a *  MatchRules::COEF_CHOIX_NOTE_IA) + (forme_a * MatchRules::COEF_CHOIX_FORME_IA);

            let note_b = b.note_actuelle.unwrap_or(0) as f32;
            let forme_b = b.forme.unwrap_or(0) as f32;
            let score_b = (note_b * MatchRules::COEF_CHOIX_NOTE_IA) + (forme_b * MatchRules::COEF_CHOIX_FORME_IA);

           
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });


        joueurs_tries.into_iter().take(11).collect()
    }

    pub fn simuler_journee(
        &self,
        matchs: &[Match],
        club_utilisateur_id: i32,
        composition_utilisateur: &CompositionMatch,
        clubs: &[Club],
        joueurs_par_club: &HashMap<i32, Vec<Joueur>>,
    ) -> Result<Vec<ResultatMatchJournee>, String> {
        let composition_manager = CompositionManager::new();
        let mut resultats = Vec::new();
        let mut joueurs_exclus = Vec::new();

        for m in matchs {
            let club_dom = clubs
                .iter()
                .find(|c| c.id == Some(m.club_domicile_id))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Club domicile introuvable pour le match {}",
                        m.id
                    )
                })?;

            let club_ext = clubs
                .iter()
                .find(|c| c.id == Some(m.club_exterieur_id))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Club extérieur introuvable pour le match {}",
                        m.id
                    )
                })?;

            let mut club_dom_sim = club_dom.clone();
            let mut club_ext_sim = club_ext.clone();

            let mut compo_dom = if m.club_domicile_id == club_utilisateur_id {
                composition_utilisateur.clone()
            } else {
                let joueurs_dom = joueurs_par_club
                    .get(&m.club_domicile_id)
                    .ok_or_else(|| {
                        format!(
                            "Joueurs domicile introuvables pour le club {}",
                            m.club_domicile_id
                        )
                    })?;

                let meilleurs_dom = self.choisir_11_meilleurs(joueurs_dom);

                if meilleurs_dom.len() < 11 {
                    return Err(format!(
                        "Le club {} n'a pas assez de joueurs pour simuler le match {}",
                        m.club_domicile_id, m.id
                    ));
                }

                composition_manager.creer_composition_match(
                    m.id,
                    m.club_domicile_id,
                    &meilleurs_dom,
                )
            };

            let mut compo_ext = if m.club_exterieur_id == club_utilisateur_id {
                composition_utilisateur.clone()
            } else {
                let joueurs_ext = joueurs_par_club
                    .get(&m.club_exterieur_id)
                    .ok_or_else(|| {
                        format!(
                            "Joueurs extérieurs introuvables pour le club {}",
                            m.club_exterieur_id
                        )
                    })?;

                let meilleurs_ext = self.choisir_11_meilleurs(joueurs_ext);

                if meilleurs_ext.len() < 11 {
                    return Err(format!(
                        "Le club {} n'a pas assez de joueurs pour simuler le match {}",
                        m.club_exterieur_id, m.id
                    ));
                }

                composition_manager.creer_composition_match(
                    m.id,
                    m.club_exterieur_id,
                    &meilleurs_ext,
                )
            };

            for j in &compo_dom.joueurs {
                joueurs_exclus.push(j.id);
            }
            for j in &compo_ext.joueurs {
                joueurs_exclus.push(j.id);
            }

            let resultat = self.simuler_match_et_sauvegarder(
                m.id,
                &mut compo_dom,
                &mut compo_ext,
                &mut club_dom_sim,
                &mut club_ext_sim,
            )?;

            resultats.push(ResultatMatchJournee {
                match_id: m.id,
                club_domicile_id: m.club_domicile_id,
                club_exterieur_id: m.club_exterieur_id,
                nom_domicile: club_dom.nom.clone(),
                nom_exterieur: club_ext.nom.clone(),
                url_logo_domicile: Some(m.club_domicile_logo.clone()), 
                url_logo_exterieur: Some(m.club_exterieur_logo.clone()),
                buts_domicile: resultat.buts_domicile,
                buts_exterieur: resultat.buts_exterieur,
                est_match_utilisateur: m.club_domicile_id == club_utilisateur_id
                    || m.club_exterieur_id == club_utilisateur_id,
            });
        }

        resultats.sort_by_key(|r| if r.est_match_utilisateur { 0 } else { 1 });

        self.appliquer_recuperation_forme_globale(&joueurs_exclus);

        Ok(resultats)
    }
}