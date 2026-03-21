use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club , Ecran};
use crate::selection_club::businessLogic::ClubFacade;
use std::sync::Arc;
use crate::page::accueil;





pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>, 
    pub facade: ClubFacade,   
}

impl MyApp {
    pub fn new(conn:  Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn);
        
        let equipes = facade.get_all().unwrap_or_else(|e| {
            println!("Erreur lors de la récupération des clubs : {:?}", e);
            vec![]
        });

        Self {
            ecran_actuel: Ecran::Accueil,
            equipe_choisie: None,
            liste_equipes: equipes,
            facade,
        }
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ecran_actuel {

                Ecran ::Accueil =>{
                    accueil::render(ui, &mut self.ecran_actuel);
                }

                Ecran::Selection => {
                    ui.heading("Sélectionnez votre club");
                    for eq in &self.liste_equipes  {
                        if ui.button(&eq.nom).clicked() {
                            self.equipe_choisie = Some(eq.clone());
                            self.ecran_actuel = Ecran::MenuPrincipal;
                        }
                    }
                }

                Ecran::MenuPrincipal => {
                    if let Some(eq) = &self.equipe_choisie {
                        ui.heading(format!("Manager de : {}", eq.nom));
                        if ui.button(" Infos Club").clicked() { self.ecran_actuel = Ecran::InfosClub; }
                        if ui.button(" Composition").clicked() { self.ecran_actuel = Ecran::Composition; }
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
            }
        });
    }
}