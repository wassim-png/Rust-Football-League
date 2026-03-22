use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke};
use crate::models::Ecran;

pub fn render(ui: &mut Ui, ecran_actuel: &mut Ecran) {
   
    egui::Image::new("file://assets/wenger.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size()) 
        .paint_at(ui, ui.max_rect());


    ui.vertical_centered(|ui| {
         ui.label(
            RichText::new("RUST FOOTBALL LEAGUE")
                .font(FontId::proportional(60.0)) 
                .strong()
                .color(Color32::RED) 
                .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 180)) 

                
        );
        ui.add_space(ui.available_height() / 2.0 - 50.0);

       

        // Style du bouton
        let bouton = egui::Button::new(
            RichText::new("PLAY")
                .font(FontId::proportional(30.0))
                .strong()
                .color(Color32::WHITE)
        )
        .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 180)) // Noir transparent
        .stroke(Stroke::new(2.0, Color32::GOLD))
        .rounding(10.0)
        .min_size(Vec2::new(220.0, 70.0));

        if ui.add(bouton).clicked() {
            *ecran_actuel = Ecran::Selection; // On change l'état
        }
    });
}