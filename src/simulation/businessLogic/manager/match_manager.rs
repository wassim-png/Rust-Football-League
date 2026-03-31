use std::sync::OnceLock;
use rusqlite::Connection;
use rand::Rng;
use std::sync::Arc;
use crate::simulation::config::match_rules::MatchRules;
use crate::simulation::persistSimulation::dao::composition_dao::CompositionDao;
use crate::simulation::persistSimulation::dao::match_dao::MatchDao;
use crate::models::Club;
use crate::models::CompositionMatch;
use crate::models::poste::Poste;
use crate::models::ResultatSimulationMatch;
use crate::simulation::persistSimulation::sqlitedao::sqlite_match_dao::SqliteMatchDao;
use crate::selection_club::persist_club::club_dao::ClubDAO;
use crate::selection_club::persist_club::sqlite_club_dao::SqliteClubDao;
"""use crate::::sqlite_composition_dao::SqliteCompositionDao;"""
pub struct MatchManager {
    match_dao:  Box<dyn MatchDao>,
    composition_dao: Box<dyn CompositionDao>,
    club_dao: Box<dyn ClubDAO>,
}





impl MatchManager {
   pub fn new(conn: Arc<Connection>, ) -> Self {
        Self {
            dao: Box::new(SqliteMatchDao { conn }),
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
            let perte = match joueur.poste {
                Poste::Gardien => MatchRules::PERTE_FORME_GARDIEN,
                Poste::Defense => MatchRules::PERTE_FORME_DEFENSE,
                Poste::Milieu => MatchRules::PERTE_FORME_MILIEU,
                Poste::Attaque => MatchRules::PERTE_FORME_ATTAQUE,
            };

            joueur.forme = (joueur.forme - perte).max(MatchRules::FORME_MIN);
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
    ) -> Result<ResultatSimulationMatch, String> {
        let match_data = self
            .match_dao
            .find_match_by_id(match_id)?
            .ok_or_else(|| format!("Aucun match trouvé avec l'id {}", match_id))?;

        let mut equipe_domicile = self
            .composition_dao
            .find_by_match_and_club(
                match_data.id,
                match_data.club_domicile_id,
                match_data.saison_id,
            )?
            .ok_or_else(|| "Composition domicile introuvable".to_string())?;

        let mut equipe_exterieur = self
            .composition_dao
            .find_by_match_and_club(
                match_data.id,
                match_data.club_exterieur_id,
                match_data.saison_id,
            )?
            .ok_or_else(|| "Composition extérieure introuvable".to_string())?;

        let mut club_domicile = self
            .club_dao
            .find_by_id(match_data.club_domicile_id)?
            .ok_or_else(|| "Club domicile introuvable".to_string())?;

        let mut club_exterieur = self
            .club_dao
            .find_by_id(match_data.club_exterieur_id)?
            .ok_or_else(|| "Club extérieur introuvable".to_string())?;

        let resultat = self.simuler_match(
            match_data.id,
            &mut equipe_domicile,
            &mut equipe_exterieur,
            &mut club_domicile,
            &mut club_exterieur,
        );

        self.match_dao.save_resultat_match(&resultat)?;
        self.club_dao.update_club(&club_domicile)?;
        self.club_dao.update_club(&club_exterieur)?;

        Ok(resultat)
    }

    pub fn update_club(&self, club: &Club) -> rusqlite::Result<()> {
  
        let id = club.id.expect("Erreur : Impossible de mettre à jour un club sans ID !");

        self.conn.execute(
            "UPDATE clubs SET 
                nom = ?1,
                nom_court = ?2,
                reputation = ?3,
                budget_eur = ?4,
                revenu_par_journee_eur = ?5,
                avantage_domicile = ?6,
                url_logo = ?7,
                points = ?8,
                buts_marques = ?9,
                buts_encaisses = ?10
            WHERE id = ?11",
            params![
                club.nom,
                club.nom_court,
                club.reputation,
                club.budget_eur,
                club.revenu_par_journee_eur,
                club.avantage_domicile,
                club.url_logo,
                club.points,
                club.buts_marques,
                club.buts_encaisses,
                id 
            ],
        )?;

        Ok(())
    }
}