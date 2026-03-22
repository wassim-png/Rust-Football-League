use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke};
use crate::models::{Club, Ecran};

pub fn render(ui: &mut Ui, clubs: &[Club], equipe_choisie: &mut Option<Club>, ecran_actuel: &mut Ecran) {
    egui::Image::new("file://assets/pelouse.jpg")
        .max_size(ui.available_size()) 
        .paint_at(ui, ui.max_rect());

    ui.vertical_centered(|ui| {
        ui.add_space(30.0);
        ui.label(
            RichText::new("CHOISISSEZ VOTRE CLUB")
                .font(FontId::proportional(40.0))
                .strong()
                .color(Color32::LIGHT_BLUE)
        );
     
        ui.add_space(20.0);
    });

   egui::ScrollArea::vertical().show(ui, |ui| {
        
     
        for groupe_de_clubs in clubs.chunks(3) {
            
            
            ui.columns(3, |colonnes| {
                
                for (index, club) in groupe_de_clubs.iter().enumerate() {
                    // On dessine la carte dans la colonne correspondante
                    render_club_card(&mut colonnes[index], club, equipe_choisie, ecran_actuel);
                }
            });
            
            ui.add_space(20.0); 
        }
    });
}

// Fonction interne pour dessiner une seule carte de club
fn render_club_card(ui: &mut Ui, club: &Club, equipe_choisie: &mut Option<Club>, ecran_actuel: &mut Ecran) {

    let is_selected = equipe_choisie.as_ref().map_or(false, |c| c.id == club.id);
    
    // Changement de couleur si sélectionné
    let bg_color = if is_selected { Color32::from_rgb(45, 60, 110) } else { Color32::from_gray(30) };
    let stroke_color = if is_selected { Color32::GOLD } else { Color32::GRAY };

    let nb_etoiles : usize = match club.reputation{
        90..=100 => 5,
            70..=89  => 4,
            60..=70 => 3,
            50..=60  => 2,
            30..=49  => 2,
            10..=29  => 1,
            _        => 0,
        };

        let etoiles_pleines = "★".repeat(nb_etoiles);
        let etoiles_vides = "☆".repeat(5 - nb_etoiles);
    


    Frame::none()
        .fill(bg_color)
        .rounding(10.0)
        .stroke(Stroke::new(2.0, stroke_color))
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.set_min_width(200.0);
            ui.vertical_centered(|ui| {
            let chemin_logo = format!("file://.{}", club.url_logo); 
        ui.add(
            egui::Image::new(&chemin_logo)
                .fit_to_exact_size(egui::vec2(60.0, 60.0)) 
                .rounding(5.0) 
        );
          
                ui.label(RichText::new(&club.nom).font(FontId::proportional(20.0)).strong());
                ui.add_space(10.0);
               
               ui.label(
            RichText::new(format!("Réputation: {}{}", etoiles_pleines, etoiles_vides))
                .color(Color32::GOLD)
        );
                ui.label(format!("Budget: {} M€", club.budget_eur / 1000000));
                ui.add_space(15.0);
                
                let texte_bouton =  "Choisir" ;
                if ui.button(texte_bouton).clicked() {
                    *equipe_choisie = Some(club.clone());
                   
                     *ecran_actuel = Ecran::MenuPrincipal; 
                }
            });
        });
}