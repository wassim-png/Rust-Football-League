

use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke, Image};
use crate::models::{Ecran, InfosClub}; 

pub fn render_info_club(
    ui: &mut Ui, 
    ecran_actuel: &mut Ecran, 
    info_club: &InfosClub, 
    reputation :&str,
    nom_club: &str 
) {
    // 1. L'IMAGE DE FOND (Le stade du club en arrière-plan)
    // Assure-toi que info_club.url_stade contienne bien "file://..." 
    // ou formate-le ici : let bg_path = format!("file://{}", info_club.url_stade);
    egui::Image::new(&info_club.url_stade)
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size()) 
        .paint_at(ui, ui.max_rect());

    // 2. LE CONTENU PAR-DESSUS (Centré)
    ui.vertical_centered(|ui| {
        ui.add_space(40.0); // Espacement en haut

        // Nom du club avec fond semi-transparent pour la lisibilité
        ui.label(
            RichText::new(nom_club)
                .font(FontId::proportional(50.0)) 
                .strong()
                .color(Color32::WHITE) 
                .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 180)) 
        );

        ui.add_space(20.0);

        // Logo du club
        ui.add(
            egui::Image::new(&info_club.url_logo)
                .max_size(Vec2::new(150.0, 150.0))
        );

        ui.add_space(30.0);

        // Nom du stade
        let texte_stade = format!("🏟️ Stade : {}", info_club.nom_stade);
        ui.label(
            RichText::new(texte_stade)
                .font(FontId::proportional(35.0))
                .strong()
                .color(Color32::LIGHT_BLUE)
                .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 180))
        );

        ui.add_space(10.0);

        // Capacité du stade
        let texte_capacite = format!("👥 Capacité : {} places", info_club.stade_capacite);
        ui.label(
            RichText::new(texte_capacite)
                .font(FontId::proportional(28.0))
                .color(Color32::GOLD)
                .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 180))
        );

        // On pousse le bouton vers le bas
        ui.add_space(ui.available_height() / 2.0 - 50.0);

        // Bouton de retour
        let bouton_retour = egui::Button::new(
            RichText::new("RETOUR")
                .font(FontId::proportional(25.0))
                .strong()
                .color(Color32::WHITE)
        )
        .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 180)) // Noir transparent
        .stroke(Stroke::new(2.0, Color32::RED))
        .rounding(10.0)
        .min_size(Vec2::new(200.0, 60.0));

        if ui.add(bouton_retour).clicked() {
            *ecran_actuel = Ecran::Selection; // Ou Ecran::Menu selon ton Enum
        }
    });
}