use eframe::egui;
use rusqlite::Connection;
use crate::models::{Club, Joueur, Ecran};
use crate::selection_club::business_logic::ClubFacade;
use crate::selection_club::ui::ecran_selection;
use crate::composition::business_logic::JoueurFacade;
use crate::composition::ui::ecran_composition;
use std::sync::Arc;
use crate::page::accueil;


pub struct MyApp {
    pub ecran_actuel: Ecran,
    pub equipe_choisie: Option<Club>,
    pub liste_equipes: Vec<Club>,
    #[allow(dead_code)]
    pub facade: ClubFacade,
    pub joueur_facade: JoueurFacade,
    pub joueurs_club: Vec<Joueur>,
    pub composition: [Option<usize>; 11],
    pub slot_actif: Option<usize>,
}

impl MyApp {
    pub fn new(conn: Arc<Connection>) -> Self {
        let facade = ClubFacade::new(conn.clone());
        let joueur_facade = JoueurFacade::new(conn);

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
            joueur_facade,
            joueurs_club: vec![],
            composition: [None; 11],
            slot_actif: None,
        }
    }

    /// Charge les joueurs du club sélectionné
    fn charger_joueurs_club(&mut self) {
        if let Some(club) = &self.equipe_choisie {
            if let Some(club_id) = club.id {
                self.joueurs_club = self.joueur_facade.get_joueurs_du_club(club_id)
                    .unwrap_or_else(|e| {
                        println!("Erreur chargement joueurs : {:?}", e);
                        vec![]
                    });
                println!("Joueurs chargés : {}", self.joueurs_club.len());
            }
        }
        self.composition = [None; 11];
        self.slot_actif = None;
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
                    let ancien_club = self.equipe_choisie.clone();
                    ecran_selection::render(ui, &self.liste_equipes, &mut self.equipe_choisie, &mut self.ecran_actuel);
                    
                    // Si un club vient d'être sélectionné, charger ses joueurs
                    let nouveau_club = self.equipe_choisie.clone();
                    if ancien_club.is_none() && nouveau_club.is_some()
                        || ancien_club.as_ref().map(|c| c.id) != nouveau_club.as_ref().map(|c| c.id)
                    {
                        self.charger_joueurs_club();
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
                    let nom_club = self.equipe_choisie.as_ref()
                        .map(|c| c.nom.clone())
                        .unwrap_or_default();
                    ecran_composition::render(
                        ui,
                        &self.joueurs_club,
                        &mut self.composition,
                        &mut self.slot_actif,
                        &mut self.ecran_actuel,
                        &nom_club,
                    );
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