use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club, Ecran, EtatMercato};
use crate::selection_club::businessLogic::ClubFacade;
use crate::selection_club::ui::ecran_selection;
use crate::mercato::businessLogic::mercato_facade::MercatoFacade;
use crate::mercato::ui::ecran_mercato;
use std::sync::Arc;
use crate::page::accueil;

pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>,
    pub facade: ClubFacade,
    pub mercato_facade: MercatoFacade,
    pub mercato: EtatMercato,
}

impl MyApp {
    pub fn new(conn: Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn.clone());
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
                    if let Some(eq) = &self.equipe_choisie {
                        ui.heading(format!("Manager de : {}", eq.nom));
                        if ui.button(" Infos Club").clicked() { self.ecran_actuel = Ecran::InfosClub; }
                        if ui.button(" Composition").clicked() { self.ecran_actuel = Ecran::Composition; }
                        if ui.button(" Marché des Transferts").clicked() {
                            self.ecran_actuel = Ecran::Mercato;
                        }
                    }
                }

                Ecran::InfosClub => {
                    if let Some(eq) = &self.equipe_choisie {
                        ui.heading("Détails du Club");
                        ui.label(format!("Nom : {}", eq.nom));
                        ui.label(format!("Budget : {} M€", eq.budget_eur));
                        if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
                    }
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

                Ecran::Mercato => {
                    if !self.mercato.donnees_chargees {
                        let club_id = self.equipe_choisie.as_ref().and_then(|c| c.id).unwrap_or(0);
                        self.mercato.tous_joueurs = self.mercato_facade
                            .get_tous_joueurs_disponibles(club_id)
                            .unwrap_or_else(|e| { println!("Erreur joueurs : {:?}", e); vec![] });
                        self.mercato.offres_recues = self.mercato_facade
                            .generer_offres_ia(club_id)
                            .unwrap_or_else(|e| { println!("Erreur offres IA : {:?}", e); vec![] });
                        self.mercato.donnees_chargees = true;
                    }
                    // Field splitting : emprunts mutables sur des champs distincts
                    if let Some(ref mut eq) = self.equipe_choisie {
                        ecran_mercato::render(ctx, ui, eq, &mut self.mercato, &mut self.ecran_actuel);
                    }
                }
            }
        });
    }
}
