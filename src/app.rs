use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club , Ecran};
use crate::selection_club::businessLogic::ClubFacade;
use crate::selection_club::ui::ecran_selection;
use crate::infos_club::ui::ecran_infos;
use std::sync::Arc;
use crate::page::accueil;





pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>, 
    pub facade: ClubFacade,   
    pub info_club_actuel: Option<InfosClub>,
}

impl MyApp {
    pub fn new(conn:  Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn);
        
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
            info_club_actuel: None,
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
               
                ecran_selection::render(ui, &self.liste_equipes, &mut self.equipe_choisie, &mut self.ecran_actuel);
            }

                Ecran::MenuPrincipal => {
                    if let Some(eq) = &self.equipe_choisie {
                        ui.heading(format!("Manager de : {}", eq.nom));
                        if ui.button(" Infos Club").clicked() { self.ecran_actuel = Ecran::InfosClub; }
                        if ui.button(" Composition").clicked() { self.ecran_actuel = Ecran::Composition; }
                        if ui.button(" Infos Club").clicked() { 
                            // On interroge la base de données une seule fois
                            match self.InfosClubfacade.(eq.id) { // ou eq.club_id selon ta struct
                                Ok(infos) => self.info_club_actuel = Some(infos),
                                Err(e) => println!("Erreur DB InfosClub : {:?}", e),
                            }
                           
                        }
                    }
                }


                Ecran::InfosClub => {
                  ecran_infos::render(ui,  infos_club, &mut self.equipe_choisie);
                   if ui.button("⬅ Retour").clicked() { self.ecran_actuel = Ecran::MenuPrincipal; }
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