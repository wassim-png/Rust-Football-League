use eframe::egui;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::{
    Club, CompositionMatch, Ecran, EtatCalendrier, EtatMercato, InfosClub, Joueur, Match,
    ResultatMatchJournee,
};

use crate::selection_club::business_logic::ClubFacade;
use crate::selection_club::ui::ecran_selection;

use crate::infos_club::businessLogic::infos_club_facade::InfosClubFacade;
use crate::infos_club::ui::ecran_infos;

use crate::prochain_match::businessLogic::next_game_facade::NextGameFacade;

use crate::mercato::businessLogic::mercato_facade::MercatoFacade;
use crate::mercato::ui::ecran_mercato;

use crate::calendrier::businessLogic::calendrier_facade::CalendrierFacade;
use crate::calendrier::ui::ecran_calendrier;

use crate::composition::business_logic::composition_facade::CompositionFacade;
use crate::composition::ui::ecran_composition;

use crate::simulation::businessLogic::facade::match_facade::MatchFacade;

use crate::page::accueil;
use crate::page::menu_principal;

pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>,

    pub club_facade: ClubFacade,
    pub mercato_facade: MercatoFacade,
    pub next_game_facade: NextGameFacade,
    pub calendrier_facade: CalendrierFacade,
    pub infos_club_facade: InfosClubFacade,
    pub composition_facade: CompositionFacade,
    pub match_facade: MatchFacade,

    pub mercato: EtatMercato,
    pub calendrier: EtatCalendrier,

    pub info_club_actuel: Option<InfosClub>,
    pub prochain_match: Option<Match>,
    pub match_deja_charge: bool,
    pub matchs_du_jour: Option<Vec<Match>>,
    pub journee_actuelle: i32,

    pub joueurs_club: Vec<Joueur>,
    pub composition: [Option<Joueur>; 11],
    pub slot_actif: Option<usize>,

    pub composition_match_actuelle: Option<CompositionMatch>,

    pub resultats_journee: Option<Vec<ResultatMatchJournee>>,
    pub simulation_deja_faite: bool,
    pub message_simulation: Option<String>,
}

impl MyApp {
    pub fn new(conn: Arc<Connection>) -> Self {
        let club_facade = ClubFacade::new(conn.clone());
        let mercato_facade = MercatoFacade::new(conn.clone());
        let next_game_facade = NextGameFacade::new(conn.clone());
        let infos_club_facade = InfosClubFacade::new(conn.clone());
        let calendrier_facade = CalendrierFacade::new(conn.clone());
        let composition_facade = CompositionFacade::new(conn.clone());
        let match_facade = MatchFacade::new(conn.clone());

        let mut calendrier = EtatCalendrier::default();

        match calendrier_facade.init_et_get_matchs() {
            Ok(matchs) => {
                calendrier.nb_journees = matchs.iter().map(|m| m.journee).max().unwrap_or(34);
                calendrier.tous_matchs = matchs;
                calendrier.donnees_chargees = true;
                println!(
                    "Calendrier préchargé au démarrage ({} matchs).",
                    calendrier.tous_matchs.len()
                );
            }
            Err(e) => {
                println!("Erreur préchargement calendrier : {:?}", e);
            }
        }

        let  equipes = club_facade.get_all().unwrap_or_else(|e| {
            println!("Erreur lors de la récupération des clubs : {:?}", e);
            vec![]
        });

        println!("Nombre de clubs chargés : {}", equipes.len());

        Self {
            ecran_actuel: Ecran::Accueil,
            equipe_choisie: None,
            liste_equipes: equipes,

            club_facade,
            mercato_facade,
            next_game_facade,
            calendrier_facade,
            infos_club_facade,
            composition_facade,
            match_facade,

            mercato: EtatMercato::default(),
            calendrier,

            info_club_actuel: None,
            prochain_match: None,
            match_deja_charge: false,
            matchs_du_jour: None,
            journee_actuelle: 1,

            joueurs_club: vec![],
            composition: std::array::from_fn(|_| None),
            slot_actif: None,

            composition_match_actuelle: None,

            resultats_journee: None,
            simulation_deja_faite: false,
            message_simulation: None,
        }
    }


    fn reset_composition_state(&mut self) {
        self.composition = std::array::from_fn(|_| None);
        self.slot_actif = None;
    }

    fn reset_simulation_state(&mut self) {
        self.resultats_journee = None;
        self.simulation_deja_faite = false;
        self.message_simulation = None;
        self.liste_equipes = self.club_facade.get_all_clubs_by_points().unwrap_or_else(|e| {
            println!("Erreur lors de la récupération des clubs : {:?}", e);
            vec![]
        });
    }

    fn charger_joueurs_pour_composition(&mut self, club_id: i32) {
        self.joueurs_club = self
            .mercato_facade
            .get_joueurs_mon_club(club_id)
            .unwrap_or_else(|e| {
                println!("Erreur chargement joueurs composition : {:?}", e);
                vec![]
            });

        self.reset_composition_state();

        println!(
            "Écran composition chargé : {} joueurs récupérés pour le club {}",
            self.joueurs_club.len(),
            club_id
        );
    }

    fn construire_joueurs_par_club(&self, matchs: &[Match]) -> HashMap<i32, Vec<Joueur>> {
        let mut joueurs_par_club = HashMap::new();

        for m in matchs {
            let ids = [m.club_domicile_id, m.club_exterieur_id];

            for club_id in ids {
                if !joueurs_par_club.contains_key(&club_id) {
                    let joueurs = self
                        .mercato_facade
                        .get_joueurs_mon_club(club_id)
                        .unwrap_or_else(|e| {
                            println!("Erreur chargement joueurs club {} : {:?}", club_id, e);
                            vec![]
                        });

                    joueurs_par_club.insert(club_id, joueurs);
                }
            }
        }

        joueurs_par_club
    }
    fn passer_a_la_journee_suivante(&mut self) {
    if self.journee_actuelle < self.calendrier.nb_journees {
        self.journee_actuelle += 1;
    }

    self.composition_match_actuelle = None;
    self.reset_composition_state();
    self.reset_simulation_state();
    // Force le rechargement du calendrier depuis la DB (pour avoir les résultats à jour)
    self.calendrier.donnees_chargees = false;

    if let Some(club_id) = self.equipe_choisie.as_ref().and_then(|c| c.id) {
        self.prochain_match = self
            .next_game_facade
            .get_next_game(club_id, self.journee_actuelle)
            .ok();

        self.matchs_du_jour = self
            .calendrier_facade
            .get_tous_matchs_par_journee(1, self.journee_actuelle)
            .ok();
    } else {
        self.prochain_match = None;
        self.matchs_du_jour = None;
    }

    self.match_deja_charge = true;
}
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ecran_actuel {
                Ecran::Accueil => {
                    accueil::render(ui, &mut self.ecran_actuel);
                }

                Ecran::Selection => {
                    ecran_selection::render(
                        ui,
                        &self.liste_equipes,
                        &mut self.equipe_choisie,
                        &mut self.ecran_actuel,
                    );
                }

                Ecran::MenuPrincipal => {
                    if let Some(ref eq) = self.equipe_choisie {
                        let club_id = eq.id.unwrap_or(0);

                        if !self.match_deja_charge {
                            self.prochain_match = self
                                .next_game_facade
                                .get_next_game(club_id, self.journee_actuelle)
                                .ok();

                            self.matchs_du_jour = self
                                .calendrier_facade
                                .get_tous_matchs_par_journee(1, self.journee_actuelle)
                                .ok();

                            println!("Matchs du jour chargés : {:#?}", self.matchs_du_jour);

                            self.match_deja_charge = true;
                        }

                        let ancien_ecran = self.ecran_actuel.clone();

                        menu_principal::render(
                            ui,
                            eq,
                            &mut self.ecran_actuel,
                            &self.prochain_match,
                            &self.liste_equipes,
                            self.journee_actuelle,
                        );

                        if !matches!(ancien_ecran, Ecran::ProchainMatch)
                            && matches!(self.ecran_actuel, Ecran::ProchainMatch)
                        {
                            self.reset_simulation_state();
                        }

                        if matches!(self.ecran_actuel, Ecran::InfosClub) {
                            match self.infos_club_facade.obtenir_infos_club(club_id) {
                                Ok(infos) => {
                                    println!("✅ BDD succès : infos récupérées !");
                                    self.info_club_actuel = Some(infos);
                                }
                                Err(e) => {
                                    println!("❌ Erreur BDD : {:?}", e);
                                    self.ecran_actuel = Ecran::MenuPrincipal;
                                }
                            }
                        }

                        if matches!(self.ecran_actuel, Ecran::Composition) {
                            self.charger_joueurs_pour_composition(club_id);
                        }
                    }
                }

                Ecran::InfosClub => {
                    if let (Some(equipe), Some(infos)) =
                        (&self.equipe_choisie, &self.info_club_actuel)
                    {
                        ecran_infos::render(ui, equipe, infos);
                    } else {
                        ui.heading("Erreur : Données du club introuvables.");
                    }

                    egui::Area::new("bouton_retour_infos")
                        .fixed_pos(egui::pos2(20.0, 20.0))
                        .show(ui.ctx(), |ui| {
                            if ui.button(egui::RichText::new("⬅ Retour").size(18.0)).clicked() {
                                self.ecran_actuel = Ecran::MenuPrincipal;
                            }
                        });
                }

                Ecran::Composition => {
                    let nom_club = self
                        .equipe_choisie
                        .as_ref()
                        .map(|c| c.nom.clone())
                        .unwrap_or_default();

                    let composition_validee = ecran_composition::render(
                        ui,
                        &self.joueurs_club,
                        &mut self.composition,
                        &mut self.slot_actif,
                        &mut self.ecran_actuel,
                        &nom_club,
                    );

                    if composition_validee {
                        if let (Some(club), Some(prochain_match)) =
                            (&self.equipe_choisie, &self.prochain_match)
                        {
                            let joueurs_selectionnes: Vec<Joueur> = self
                                .composition
                                .iter()
                                .filter_map(|slot| slot.clone())
                                .collect();

                            let composition_match =
                                self.composition_facade.creer_composition_match(
                                    prochain_match.id,
                                    club.id.unwrap_or(0),
                                    &joueurs_selectionnes,
                                );

                            println!("Composition créée : {:#?}", composition_match);

                            self.composition_match_actuelle = Some(composition_match);
                            self.ecran_actuel = Ecran::MenuPrincipal;
                        }
                    }
                }

                Ecran::DetailsJoueur => {
                    ui.heading("Détail du joueur");
                    ui.label("On affiche ses attributs");
                    if ui.button("⬅ Retour").clicked() {
                        self.ecran_actuel = Ecran::MenuPrincipal;
                    }
                }

                Ecran::Calendrier => {
                    if !self.calendrier.donnees_chargees {
                        self.calendrier.tous_matchs = self
                            .calendrier_facade
                            .init_et_get_matchs()
                            .unwrap_or_else(|e| {
                                println!("Erreur calendrier : {:?}", e);
                                vec![]
                            });

                        self.calendrier.nb_journees = 34;
                        self.calendrier.donnees_chargees = true;
                        // Ouvre toujours sur la journée actuelle
                        self.calendrier.journee_selectionnee = self.journee_actuelle;
                    }

                    let club_id = self.equipe_choisie.as_ref().and_then(|c| c.id).unwrap_or(0);

                    ecran_calendrier::render(
                        ui,
                        &mut self.calendrier,
                        club_id,
                        &mut self.ecran_actuel,
                    );
                }

                Ecran::Classement => {
                    ui.heading("Classement Ligue 1");
                    ui.label("La table du championnat s'affichera ici...");
                    if ui.button("⬅ Retour").clicked() {
                        self.ecran_actuel = Ecran::MenuPrincipal;
                    }
                }

                Ecran::ProchainMatch => {
                    if !self.simulation_deja_faite {
                        if self.composition_match_actuelle.is_none() {
                            self.message_simulation = Some(
                                "Vous devez valider votre composition avant de lancer la simulation."
                                    .to_string(),
                            );
                        } else if self.matchs_du_jour.is_none() {
                            self.message_simulation =
                                Some("Aucun match de la journée n'est chargé.".to_string());
                        } else if self.equipe_choisie.is_none() {
                            self.message_simulation = Some("Aucune équipe choisie.".to_string());
                        } else {
                            let composition_utilisateur =
                                self.composition_match_actuelle.clone().unwrap();

                            let club_utilisateur_id = self
                                .equipe_choisie
                                .as_ref()
                                .and_then(|c| c.id)
                                .unwrap_or(0);

                            let matchs = self.matchs_du_jour.clone().unwrap_or_default();
                            let joueurs_par_club = self.construire_joueurs_par_club(&matchs);

                            match self.match_facade.simuler_journee(
                                &matchs,
                                club_utilisateur_id,
                                &composition_utilisateur,
                                &self.liste_equipes,
                                &joueurs_par_club,
                            ) {
                                Ok(resultats) => {
                                    self.resultats_journee = Some(resultats);
                                    self.message_simulation = None;
                                }
                                Err(e) => {
                                    self.resultats_journee = None;
                                    self.message_simulation =
                                        Some(format!("Erreur simulation : {}", e));
                                }
                            }
                        }

                        self.simulation_deja_faite = true;
                        
                    }

                    ui.heading("Résultats de la journée");
                    ui.add_space(10.0);

                    if let Some(message) = &self.message_simulation {
                        ui.label(message);
                        ui.add_space(10.0);
                    }

                    if let Some(resultats) = &self.resultats_journee {
                        for resultat in resultats {
                            ui.group(|ui| {
                                if resultat.est_match_utilisateur {
                                    ui.heading("Votre match");
                                }

                                ui.label(format!(
                                    "{} {} - {} {}",
                                    resultat.nom_domicile,
                                    resultat.buts_domicile,
                                    resultat.buts_exterieur,
                                    resultat.nom_exterieur
                                ));
                            });

                            ui.add_space(8.0);
                        }
                    }

                    if ui.button("⬅ Retour").clicked() {
                        self.ecran_actuel = Ecran::MenuPrincipal;
                    }
                    ui.add_space(10.0);
                        if self.resultats_journee.is_some() && self.message_simulation.is_none() {
                            if self.journee_actuelle < self.calendrier.nb_journees {
                                if ui.button("➡ Passer à la prochaine journée").clicked() {
                                    self.passer_a_la_journee_suivante();
                                    self.ecran_actuel = Ecran::MenuPrincipal;
                                }
                            } else {
                                ui.label("🏁 Fin de saison");
                            }
                        }
                }

                Ecran::Mercato => {
                    if !self.mercato.donnees_chargees {
                        let club_id = self.equipe_choisie.as_ref().and_then(|c| c.id).unwrap_or(0);

                        self.mercato.tous_joueurs = self
                            .mercato_facade
                            .get_tous_joueurs_disponibles(club_id)
                            .unwrap_or_else(|e| {
                                println!("Erreur joueurs : {:?}", e);
                                vec![]
                            });

                        self.mercato.mes_joueurs = self
                            .mercato_facade
                            .get_joueurs_mon_club(club_id)
                            .unwrap_or_else(|e| {
                                println!("Erreur mes joueurs : {:?}", e);
                                vec![]
                            });

                        self.mercato.offres_recues = self
                            .mercato_facade
                            .generer_offres_ia(club_id)
                            .unwrap_or_else(|e| {
                                println!("Erreur offres IA : {:?}", e);
                                vec![]
                            });

                        self.mercato.donnees_chargees = true;
                    }

                    if let Some(ref mut eq) = self.equipe_choisie {
                        ecran_mercato::render(
                            ctx,
                            ui,
                            eq,
                            &mut self.mercato,
                            &mut self.ecran_actuel,
                        );
                    }

                    if let Some((joueur_id, club_id)) = self.mercato.action_recrutement.take() {
                        if let Err(e) = self.mercato_facade.recruter_joueur(joueur_id, club_id) {
                            println!("Erreur DB recrutement : {:?}", e);
                        }
                    }

                    if let Some((joueur_id, nouveau_club_id)) = self.mercato.action_vente.take() {
                        if let Err(e) =
                            self.mercato_facade.vendre_joueur(joueur_id, nouveau_club_id)
                        {
                            println!("Erreur DB vente : {:?}", e);
                        }
                    }
                }
            }
        });
    }
}