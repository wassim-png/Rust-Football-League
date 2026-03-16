use eframe::egui;

use crate::models::{Equipe, Ecran};





pub struct MyApp {
    ecran_actuel: Ecran,
    equipe_choisie: Option<Equipe>,
    base_de_donnees_temporaire: Vec<Equipe>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ecran_actuel: Ecran::Selection,
            equipe_choisie: None,
            
            base_de_donnees_temporaire: vec![
                Equipe { id: 1, nom: "PSG".into(), stade: "Parc des Princes".into(), budget: 700},
                Equipe { id: 2, nom: "OM".into(), stade: "Vélodrome".into(), budget: 300 },
                Equipe { id: 3, nom: "OL".into(), stade: "Groupama Stadium".into(), budget: 250 },
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.ecran_actuel {

                Ecran::Selection => {
                    ui.heading("Sélectionnez votre club");
                    for eq in &self.base_de_donnees_temporaire {
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
                        ui.label(format!("Stade : {}", eq.stade));
                        ui.label(format!("Budget : {} M€", eq.budget));
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