use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke, Align2};
use crate::models::Ecran;

pub fn render(ui: &mut Ui, ecran_actuel: &mut Ecran) {
    let rect = ui.max_rect();
    let cx = rect.center().x;

    // Image de fond
    egui::Image::new("file://assets/wenger.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, rect);

    // Overlay sombre
    ui.painter().rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 125));

    // --- Bloc titre ---
    let panel_w = (rect.width() * 0.80).min(680.0);
    let title_center_y = rect.top() + rect.height() * 0.22;
    let title_rect = egui::Rect::from_center_size(
        egui::pos2(cx, title_center_y),
        Vec2::new(panel_w, 130.0),
    );

    // Fond du panneau titre
    ui.painter().rect_filled(
        title_rect,
        6.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 165),
    );

    // Lignes dorées haut et bas
    ui.painter().line_segment(
        [title_rect.left_top() + Vec2::new(20.0, 0.0), title_rect.right_top() + Vec2::new(-20.0, 0.0)],
        Stroke::new(2.5, Color32::GOLD),
    );
    ui.painter().line_segment(
        [title_rect.left_bottom() + Vec2::new(20.0, 0.0), title_rect.right_bottom() + Vec2::new(-20.0, 0.0)],
        Stroke::new(2.5, Color32::GOLD),
    );

    // Titre principal
    ui.painter().text(
        egui::pos2(cx, title_rect.center().y - 20.0),
        Align2::CENTER_CENTER,
        "RUST FOOTBALL LEAGUE",
        FontId::proportional(58.0),
        Color32::WHITE,
    );

    // Sous-titre doré
    ui.painter().text(
        egui::pos2(cx, title_rect.center().y + 36.0),
        Align2::CENTER_CENTER,
        "LIGUE 1  \u{2022}  SAISON 2025 - 2026",
        FontId::proportional(17.0),
        Color32::GOLD,
    );

    // --- Bouton PLAY positionné à 75% de la hauteur ---
    let btn_rect = egui::Rect::from_center_size(
        egui::pos2(cx, rect.top() + rect.height() * 0.76),
        Vec2::new(260.0, 68.0),
    );

    ui.allocate_ui_at_rect(btn_rect, |ui| {
        ui.centered_and_justified(|ui| {
            let bouton = egui::Button::new(
                RichText::new("JOUER")
                    .font(FontId::proportional(30.0))
                    .strong()
                    .color(Color32::from_rgb(15, 15, 15)),
            )
            .fill(Color32::GOLD)
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
            .rounding(10.0)
            .min_size(Vec2::new(260.0, 68.0));

            if ui.add(bouton).clicked() {
                *ecran_actuel = Ecran::Selection;
            }
        });
    });
}
