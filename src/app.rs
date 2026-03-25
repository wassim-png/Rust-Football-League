use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club, Ecran, EtatMercato, EtatCalendrier};
use crate::selection_club::businessLogic::ClubFacade;
use crate::selection_club::ui::ecran_selection;
use crate::infos_club::ui::ecran_infos;
use crate::mercato::businessLogic::mercato_facade::MercatoFacade;
use crate::mercato::ui::ecran_mercato;
use crate::calendrier::businessLogic::calendrier_facade::CalendrierFacade;
use crate::calendrier::ui::ecran_calendrier;
use std::sync::Arc;
use crate::page::accueil;
use crate::page::menu_principal;

pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>,
    pub facade: ClubFacade,
    pub mercato_facade: MercatoFacade,
    pub mercato: EtatMercato,
    pub calendrier_facade: CalendrierFacade,
    pub calendrier: EtatCalendrier,
}

impl MyApp {
    pub fn new(conn: Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn.clone());
        let mercato_facade = MercatoFacade::new(conn.clone());

        let equipes = facade.get_all().unwrap_or_else(|e| {
            println!("Erreur lors de la récupération des clubs : {:?}", e);
            vec![]
        });
        println!("Nombre de clubs chargés : {}", equipes.len());

        Self {
            ecran_actuel: Ecran::Accueil,
            equipe_choisie: None,
            liste_equipes: equipes,
            facade,
            mercato_facade,
            mercato: EtatMercato::default(),
            calendrier_facade: CalendrierFacade::new(conn),
            calendrier: EtatCalendrier::default(),
        }
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
                    ecran_selection::render(ui, &self.liste_equipes, &mut self.equipe_choisie, &mut self.ecran_actuel);
                }

                Ecran::MenuPrincipal => {
                    if let Some(ref eq) = self.equipe_choisie {
                        menu_principal::render(ui, eq, &mut self.ecran_actuel);
                    }
                }

                Ecran::InfosClub => {
                    ecran_selection::render(ui, &self.liste_equipes, &mut self.equipe_choisie, &mut self.ecran_actuel);
                }

                Ecran::Composition => {
                    ui.heading("Ma Composition");
                    ui.label("Ici s'affichera la liste des joueurs...");
                    if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
                }

                Ecran::DetailsJoueur => {
                    ui.heading("Détail du joueur ");
                    ui.label("On affiche ses attributs");
                    if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
                }

                Ecran::Calendrier => {
                    if !self.calendrier.donnees_chargees {
                        self.calendrier.tous_matchs = self.calendrier_facade
                            .init_et_get_matchs()
                            .unwrap_or_else(|e| { println!("Erreur calendrier : {:?}", e); vec![] });
                        self.calendrier.nb_journees = 34;
                        self.calendrier.donnees_chargees = true;
                    }
                    let club_id = self.equipe_choisie.as_ref().and_then(|c| c.id).unwrap_or(0);
                    ecran_calendrier::render(ui, &mut self.calendrier, club_id, &mut self.ecran_actuel);
                }

                Ecran::Classement => {
                    ui.heading("Classement Ligue 1");
                    ui.label("La table du championnat s'affichera ici...");
                    if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
                }

                Ecran::ProchainMatch => {
                    ui.heading("Prochain Match");
                    ui.label("La simulation du prochain match s'affichera ici...");
                    if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
                }

                Ecran::Mercato => {
                    if !self.mercato.donnees_chargees {
                        let club_id = self.equipe_choisie.as_ref().and_then(|c| c.id).unwrap_or(0);
                        self.mercato.tous_joueurs = self.mercato_facade
                            .get_tous_joueurs_disponibles(club_id)
                            .unwrap_or_else(|e| { println!("Erreur joueurs : {:?}", e); vec![] });
                        self.mercato.mes_joueurs = self.mercato_facade
                            .get_joueurs_mon_club(club_id)
                            .unwrap_or_else(|e| { println!("Erreur mes joueurs : {:?}", e); vec![] });
                        self.mercato.offres_recues = self.mercato_facade
                            .generer_offres_ia(club_id)
                            .unwrap_or_else(|e| { println!("Erreur offres IA : {:?}", e); vec![] });
                        self.mercato.donnees_chargees = true;
                    }
                    // Field splitting : emprunts mutables sur des champs distincts
                    if let Some(ref mut eq) = self.equipe_choisie {
                        ecran_mercato::render(ctx, ui, eq, &mut self.mercato, &mut self.ecran_actuel);
                    }
                    // Persister le recrutement en DB après le render
                    if let Some((joueur_id, club_id)) = self.mercato.action_recrutement.take() {
                        if let Err(e) = self.mercato_facade.recruter_joueur(joueur_id, club_id) {
                            println!("Erreur DB recrutement : {:?}", e);
                        }
                    }
                    // Persister la vente en DB après le render
                    if let Some((joueur_id, nouveau_club_id)) = self.mercato.action_vente.take() {
                        if let Err(e) = self.mercato_facade.vendre_joueur(joueur_id, nouveau_club_id) {
                            println!("Erreur DB vente : {:?}", e);
                        }
                    }
                }
            }
        });
    }
}
