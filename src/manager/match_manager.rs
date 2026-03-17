use std::sync::OnceLock;

use rand::Rng;

use crate::dao::composition_dao::CompositionDao;
use crate::dao::match_dao::MatchDao;
use crate::model::composition_match::CompositionMatch;
use crate::model::resultat_match::ResultatSimulationMatch;
use crate::sqlitedao::sqlite_composition_dao::SqliteCompositionDao;
use crate::sqlitedao::sqlite_match_dao::SqliteMatchDao;

const COEF_NOTE_GENERALE: f32 = 0.5;
const COEF_NOTE_COLLECTIF: f32 = 0.3;
const COEF_FORME_GENERALE: f32 = 0.2;
const COEF_REGULATEUR: f32 = 1.0;
const COEF_REGULATEUR_BUT: f32 = 1.0;


pub struct MatchManager {
    match_dao: SqliteMatchDao,
    composition_dao: SqliteCompositionDao,
}

static INSTANCE: OnceLock<MatchManager> = OnceLock::new();

impl MatchManager {
    pub fn get_instance() -> &'static MatchManager {
        INSTANCE.get_or_init(|| MatchManager {
            match_dao: SqliteMatchDao::new(),
            composition_dao: SqliteCompositionDao::new(),
        })
    }

    pub fn calculer_note_globale(&self, equipe: &CompositionMatch) -> f32 {
        COEF_FORME_GENERALE * equipe.forme_generale
            + COEF_NOTE_GENERALE * equipe.note_generale
            + COEF_NOTE_COLLECTIF * equipe.note_collectif
    }

    pub fn calcul_cote_match(
        &self,
        equipe1: &CompositionMatch,
        equipe2: &CompositionMatch,
    ) -> (f32, f32) {
        let res = self.calculer_note_globale(equipe1) - self.calculer_note_globale(equipe2);

        let coef_eq1 = (50.0 + res * COEF_REGULATEUR).clamp(5.0, 95.0);
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
        (finition * COEF_REGULATEUR_BUT ).clamp(5.0, 90.0)
    }

    pub fn simuler_score(
        &self,
        equipe_domicile: &CompositionMatch,
        equipe_exterieur: &CompositionMatch,
    ) -> (i32, i32) {
        let (cote1, cote2) = self.calcul_cote_match(equipe_domicile, equipe_exterieur);

        let occasions1 = self.calcul_occasions(cote1);
        let occasions2 = self.calcul_occasions(cote2);

        let proba1 = self.calcul_proba_conversion(equipe_domicile.finition);
        let proba2 = self.calcul_proba_conversion(equipe_exterieur.finition);

        let mut buts1 = 0;
        let mut buts2 = 0;

        let mut rng = rand::thread_rng();

        for _ in 0..occasions1 {
            let x = rng.gen_range(0.0..100.0);
            if x < proba1 {
                buts1 += 1;
            }
        }

        for _ in 0..occasions2 {
            let x = rng.gen_range(0.0..100.0);
            if x < proba2 {
                buts2 += 1;
            }
        }

        (buts1, buts2)
    }

    pub fn simuler_match(
        &self,
        match_id: i32,
        equipe_domicile: &CompositionMatch,
        equipe_exterieur: &CompositionMatch,
    ) -> ResultatSimulationMatch {
        let (buts_dom, buts_ext) = self.simuler_score(equipe_domicile, equipe_exterieur);

        let vainqueur_id = if buts_dom > buts_ext {
            Some(equipe_domicile.club_id)
        } else if buts_ext > buts_dom {
            Some(equipe_exterieur.club_id)
        } else {
            None
        };

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

        let equipe_domicile = self
            .composition_dao
            .find_by_match_and_club(
                match_data.id,
                match_data.club_domicile_id,
                match_data.saison_id,
            )?
            .ok_or_else(|| "Composition domicile introuvable".to_string())?;

        let equipe_exterieur = self
            .composition_dao
            .find_by_match_and_club(
                match_data.id,
                match_data.club_exterieur_id,
                match_data.saison_id,
            )?
            .ok_or_else(|| "Composition extérieure introuvable".to_string())?;

        let resultat = self.simuler_match(match_data.id, &equipe_domicile, &equipe_exterieur);

        self.match_dao.save_resultat_match(&resultat)?;

        Ok(resultat)
    }
}