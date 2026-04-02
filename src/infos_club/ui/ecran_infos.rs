

use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2};
use crate::models::{ Club, InfosClub}; 

pub fn render(
    ui: &mut Ui, 
    equipe_choisie: &Club, 
    info_club: &InfosClub
) {
    let rect = ui.max_rect();
    egui::Image::new("file://assets/fond_info_club.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, rect);
    
    let chemin_logo = format!("file:/{}", info_club.url_logo);
    let chemin_stade = format!("file:/{}", info_club.url_stade);
     


    ui.vertical_centered(|ui| {
        
        ui.add_space(20.0);
        ui.heading(
            RichText::new(&equipe_choisie.nom)
                .font(FontId::proportional(45.0))
                .strong()
                .color(Color32::WHITE)
        );
        
        ui.add_space(10.0);
        ui.add(
            egui::Image::new(&chemin_logo)
                .max_size(Vec2::new(100.0, 100.0)) 
        );
        
        ui.add_space(30.0);

        
        
        
        ui.label(
            RichText::new(format!(" {}", info_club.nom_stade))
                .font(FontId::proportional(35.0))
                .strong()
                .color(Color32::LIGHT_BLUE)
        );
        
        ui.add_space(15.0);
        
       
        ui.add(
            egui::Image::new(&chemin_stade)
                .max_size(Vec2::new(800.0, 500.0)) 
                .rounding(15.0) 
        );
        
        ui.add_space(10.0);
        

        let texte_capacite = format!("👥 Capacité : {} places", info_club.stade_capacite);
        ui.label(
            RichText::new(texte_capacite)
                .font(FontId::proportional(24.0))
                .color(Color32::GRAY)
        );

        ui.add_space(40.0);

       
        ui.columns(3, |cols| {
            cols[0].vertical_centered(|ui| {
                ui.label(RichText::new("⭐ Réputation").font(FontId::proportional(20.0)).color(Color32::YELLOW));
                ui.label(RichText::new(format!("{}/100", info_club.reputation)).font(FontId::proportional(24.0)).strong());
            });
            
            cols[1].vertical_centered(|ui| {
                ui.label(RichText::new("💰 Revenus").font(FontId::proportional(20.0)).color(Color32::GOLD));
                ui.label(RichText::new(format!("{} € / match", info_club.revenu_par_journee_eur)).font(FontId::proportional(24.0)).strong());
            });
            
            cols[2].vertical_centered(|ui| {
                ui.label(RichText::new("⚽ Meilleur Buteur").font(FontId::proportional(20.0)).color(Color32::LIGHT_GREEN));
                ui.label(RichText::new(info_club.nom_meilleur_buteur.clone()).font(FontId::proportional(24.0)).strong()); 
            });
        });
    });
}