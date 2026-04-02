use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2, Align2};
use crate::models::{Club, Ecran};

pub fn render(ui: &mut Ui, clubs: &[Club], equipe_choisie: &mut Option<Club>, ecran_actuel: &mut Ecran) {
    let rect = ui.max_rect();
    let cx = rect.center().x;

    
    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, rect);

    
    ui.painter().rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 150));

    
    let header_h = 88.0;
    let header_rect = egui::Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_h));
    ui.painter().rect_filled(header_rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 175));

    
    let line_w = (rect.width() * 0.55).min(550.0);
    ui.painter().line_segment(
        [egui::pos2(cx - line_w / 2.0, header_rect.bottom()), egui::pos2(cx + line_w / 2.0, header_rect.bottom())],
        Stroke::new(2.5, Color32::GOLD),
    );

    ui.painter().text(
        egui::pos2(cx, header_rect.center().y - 10.0),
        Align2::CENTER_CENTER,
        "CHOISISSEZ VOTRE CLUB",
        FontId::proportional(42.0),
        Color32::WHITE,
    );
    ui.painter().text(
        egui::pos2(cx, header_rect.center().y + 24.0),
        Align2::CENTER_CENTER,
        "18 CLUBS EN COMPÉTITION  \u{2022}  LIGUE 1",
        FontId::proportional(14.0),
        Color32::GOLD,
    );

    
    ui.add_space(header_h + 12.0);

    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for groupe in clubs.chunks(3) {
            ui.columns(3, |colonnes| {
                for (i, club) in groupe.iter().enumerate() {
                    render_club_card(&mut colonnes[i], club, equipe_choisie, ecran_actuel);
                }
            });
            ui.add_space(12.0);
        }
        ui.add_space(20.0);
    });
}

fn render_club_card(ui: &mut Ui, club: &Club, equipe_choisie: &mut Option<Club>, ecran_actuel: &mut Ecran) {
    let is_selected = equipe_choisie.as_ref().map_or(false, |c| c.id == club.id);

    let bg_color = if is_selected {
        Color32::from_rgba_unmultiplied(28, 52, 115, 235)
    } else {
        Color32::from_rgba_unmultiplied(12, 18, 38, 218)
    };
    let (stroke_w, stroke_col) = if is_selected {
        (2.5, Color32::GOLD)
    } else {
        (1.0, Color32::from_white_alpha(35))
    };

    let nb_etoiles: usize = match club.reputation {
        90..=100 => 5,
            70..=89  => 4,
            60..=69  => 3,
            50..=59  => 2,
            30..=49  => 2,
            10..=29  => 1,
            _        => 0,
        };

        let etoiles = format!("{}{}", "★".repeat(nb_etoiles), "☆".repeat(5 - nb_etoiles));
    


    Frame::none()
        .fill(bg_color)
        .rounding(12.0)
        .stroke(Stroke::new(stroke_w, stroke_col))
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

               
                let chemin_logo = format!("file://.{}", club.url_logo);
                ui.add(
                    egui::Image::new(&chemin_logo)
                        .fit_to_exact_size(egui::vec2(75.0, 75.0))
                        .rounding(6.0),
                );
                ui.add_space(8.0);

               
                ui.label(
                    RichText::new(&club.nom)
                        .font(FontId::proportional(17.0))
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(6.0);

                // Étoiles
                ui.label(
                    RichText::new(&etoiles)
                        .font(FontId::proportional(17.0))
                        .color(Color32::GOLD),
                );

                
                ui.label(
                    RichText::new(format!("{} M€", club.budget_eur / 1_000_000))
                        .font(FontId::proportional(13.5))
                        .color(Color32::from_rgb(150, 220, 150)),
                );

                ui.add_space(10.0);

               
                let (btn_fill, btn_text_col, label) = if is_selected {
                    (Color32::GOLD, Color32::from_rgb(15, 15, 15), "SÉLECTIONNÉ  ✓")
                } else {
                    (Color32::from_rgba_unmultiplied(45, 75, 155, 210), Color32::WHITE, "CHOISIR")
                };

                let btn = egui::Button::new(
                    RichText::new(label)
                        .font(FontId::proportional(13.0))
                        .strong()
                        .color(btn_text_col),
                )
                .fill(btn_fill)
                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                .rounding(8.0)
                .min_size(Vec2::new(ui.available_width() - 4.0, 32.0));

                if ui.add(btn).clicked() {
                    *equipe_choisie = Some(club.clone());
                    *ecran_actuel = Ecran::MenuPrincipal;
                }

                ui.add_space(4.0);
            });
        });
}
