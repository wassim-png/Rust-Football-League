use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club, Ecran, EtatMercato, InfosClub};
use crate::selection_club::businessLogic::ClubFacade;
use crate::selection_club::ui::ecran_selection;
use crate::infos_club::ui::ecran_infos;
use crate::mercato::businessLogic::mercato_facade::MercatoFacade;
use crate::mercato::ui::ecran_mercato;
use crate::infos_club::businessLogic::infos_club_facade::InfosClubFacade;
use std::sync::Arc;
use crate::page::accueil;
use crate::page::menu_principal;
use crate::app::egui::RichText;
use crate::app::egui::Vec2;
use crate::app::egui::Stroke;
use crate::app::egui::Color32;
use crate::app::egui::FontId;

pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>,
    pub facade: ClubFacade,
    pub mercato_facade: MercatoFacade,
    pub mercato: EtatMercato,
    pub info_club_actuel: Option<InfosClub>,
    pub facade_infos_club: InfosClubFacade,
}

impl MyApp {
    pub fn new(conn: Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn.clone());
        let facade_infos_club = InfosClubFacade::new(conn.clone());
        let mercato_facade = MercatoFacade::new(conn);

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
            info_club_actuel: None,
            facade_infos_club
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
                    
                    
                    if let Some(ref eq) = self.equipe_choisie {
                       
                        menu_principal::render(ui, eq, &mut self.ecran_actuel);

                      
                        if matches!(self.ecran_actuel, Ecran::InfosClub) {
                            
                            match self.facade_infos_club.obtenir_infos_club(eq.id.unwrap_or(0)) { 
                                Ok(infos) => {
                                    println!("✅ BDD succès : infos récupérées !");
                                    self.info_club_actuel = Some(infos);
                                },
                                Err(e) => {
                                    println!("❌ Erreur BDD : {:?}", e);
                                    // En cas d'erreur, on annule le changement d'écran
                                    self.ecran_actuel = Ecran::MenuPrincipal; 
                                }
                            }
                        }
                    
                    }
                }
                }


               Ecran::InfosClub => {
                
                    if let (Some(equipe), Some(infos)) = (&self.equipe_choisie, &self.info_club_actuel) {
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
                    ui.heading("Calendrier");
                    ui.label("Les matchs de la saison s'afficheront ici...");
                    if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
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
