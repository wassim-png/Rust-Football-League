use eframe::egui;
use egui::{Align2, Color32, FontId, Frame, RichText, Stroke, Ui, Vec2};
use crate::models::{Ecran, EtatCalendrier};

pub fn render(ui: &mut Ui, etat: &mut EtatCalendrier, mon_club_id: i32, ecran_actuel: &mut Ecran) {
    let rect = ui.max_rect();
    let cx = rect.center().x;

   
    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, rect);
    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 170));

    let header_h = 88.0;
    let header_rect = egui::Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_h));
    ui.painter()
        .rect_filled(header_rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 190));
    let line_w = (rect.width() * 0.55).min(560.0);
    ui.painter().line_segment(
        [
            egui::pos2(cx - line_w / 2.0, header_rect.bottom()),
            egui::pos2(cx + line_w / 2.0, header_rect.bottom()),
        ],
        Stroke::new(2.5, Color32::GOLD),
    );
    ui.painter().text(
        egui::pos2(cx, header_rect.center().y - 12.0),
        Align2::CENTER_CENTER,
        "CALENDRIER",
        FontId::proportional(40.0),
        Color32::WHITE,
    );
    ui.painter().text(
        egui::pos2(cx, header_rect.center().y + 24.0),
        Align2::CENTER_CENTER,
        "LIGUE 1  \u{2022}  SAISON 2025 - 2026",
        FontId::proportional(13.5),
        Color32::GOLD,
    );

    
    ui.allocate_ui_at_rect(
        egui::Rect::from_min_size(
            header_rect.min + Vec2::new(14.0, 28.0),
            Vec2::new(95.0, 32.0),
        ),
        |ui| {
            if ui
                .add(
                    egui::Button::new(
                        RichText::new("⬅ Retour")
                            .font(FontId::proportional(13.0))
                            .color(Color32::WHITE),
                    )
                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 18))
                    .stroke(Stroke::new(1.0, Color32::from_white_alpha(60)))
                    .rounding(6.0),
                )
                .clicked()
            {
                *ecran_actuel = Ecran::MenuPrincipal;
            }
        },
    );

    // Réserver l'espace du header dans le layout
    ui.add_space(header_h + 10.0);

    // --- Données journée ---
    let matchs_journee: Vec<_> = etat
        .tous_matchs
        .iter()
        .filter(|m| m.journee == etat.journee_selectionnee)
        .cloned()
        .collect();

    let date_str = matchs_journee
        .first()
        .and_then(|m| m.date_coup_envoi.clone())
        .unwrap_or_default();

    let j = etat.journee_selectionnee;
    let nb_j = etat.nb_journees;


    let mut go_prev = false;
    let mut go_next = false;

    ui.horizontal(|ui| {
        let btn_w = 46.0;
        let btn_h = 46.0;
        let center_w = 300.0;
        
        let side = (ui.available_width() - btn_w * 2.0 - center_w).max(0.0) / 2.0;

        ui.add_space(side);

        // Flèche gauche
        let prev_col = if j > 1 { Color32::WHITE } else { Color32::from_white_alpha(35) };
        if ui
            .add_sized(
                [btn_w, btn_h],
                egui::Button::new(RichText::new("◀").font(FontId::proportional(18.0)).color(prev_col))
                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 18))
                    .stroke(Stroke::new(1.0, Color32::from_white_alpha(40)))
                    .rounding(8.0),
            )
            .clicked()
            && j > 1
        {
            go_prev = true;
        }

        // Bloc texte central (taille fixe)
        ui.allocate_ui_with_layout(
            Vec2::new(center_w, btn_h),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("JOURNÉE  {}", j))
                        .font(FontId::proportional(20.0))
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.label(
                    RichText::new(&date_str)
                        .font(FontId::proportional(12.5))
                        .color(Color32::GOLD),
                );
            },
        );

        
        let next_col = if j < nb_j { Color32::WHITE } else { Color32::from_white_alpha(35) };
        if ui
            .add_sized(
                [btn_w, btn_h],
                egui::Button::new(RichText::new("▶").font(FontId::proportional(18.0)).color(next_col))
                    .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 18))
                    .stroke(Stroke::new(1.0, Color32::from_white_alpha(40)))
                    .rounding(8.0),
            )
            .clicked()
            && j < nb_j
        {
            go_next = true;
        }
    });

 
    if go_prev { etat.journee_selectionnee -= 1; }
    if go_next { etat.journee_selectionnee += 1; }

    ui.add_space(12.0);

    // --- Liste des matchs 
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Mesurer la largeur disponible UNE fois, avant les frames
        let card_w = ui.available_width();
        let inner_margin = 10.0_f32;
        let inner_w = card_w - inner_margin * 2.0;

        for m in &matchs_journee {
            let est_mon_match =
                m.club_domicile_id == mon_club_id || m.club_exterieur_id == mon_club_id;

            Frame::none()
                .fill(if est_mon_match {
                    Color32::from_rgba_unmultiplied(18, 46, 96, 230)
                } else {
                    Color32::from_rgba_unmultiplied(10, 14, 28, 210)
                })
                .rounding(10.0)
                .stroke(Stroke::new(
                    if est_mon_match { 2.0 } else { 1.0 },
                    if est_mon_match { Color32::GOLD } else { Color32::from_white_alpha(22) },
                ))
                .inner_margin(inner_margin)
                .show(ui, |ui| {
                   
                    ui.set_min_width(inner_w);
                    render_ligne_match(ui, m, mon_club_id, inner_w);
                });

            ui.add_space(5.0);
        }
        ui.add_space(20.0);
    });
}

fn render_ligne_match(ui: &mut Ui, m: &crate::models::Match, mon_club_id: i32, total_w: f32) {
    let score_w = 90.0;
    let team_w = (total_w - score_w) / 2.0;

    ui.horizontal(|ui| {
        // Colonne DOM — right_to_left sur largeur fixe
        ui.allocate_ui_with_layout(
            Vec2::new(team_w, 48.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                if m.club_domicile_id == mon_club_id {
                    ui.label(
                        RichText::new(" DOM ")
                            .font(FontId::proportional(9.5))
                            .color(Color32::from_rgb(80, 210, 110)),
                    );
                }
                let col = if m.club_domicile_id == mon_club_id { Color32::GOLD } else { Color32::WHITE };
                ui.label(
                    RichText::new(&m.club_domicile_nom)
                        .font(FontId::proportional(13.5))
                        .strong()
                        .color(col),
                );
                ui.add(
                    egui::Image::new(format!("file://.{}", m.club_domicile_logo))
                        .fit_to_exact_size(egui::vec2(36.0, 36.0))
                        .rounding(4.0),
                );
            },
        );

        
        ui.allocate_ui_with_layout(
            Vec2::new(score_w, 48.0),
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                let (txt, col) = match (m.buts_domicile, m.buts_exterieur) {
                    (Some(d), Some(e)) => (format!("{} – {}", d, e), Color32::from_rgb(220, 220, 220)),
                    _ => ("vs".to_string(), Color32::from_white_alpha(100)),
                };
                ui.label(
                    RichText::new(txt)
                        .font(FontId::proportional(17.0))
                        .strong()
                        .color(col),
                );
            },
        );

       
        ui.allocate_ui_with_layout(
            Vec2::new(team_w, 48.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add(
                    egui::Image::new(format!("file://.{}", m.club_exterieur_logo))
                        .fit_to_exact_size(egui::vec2(36.0, 36.0))
                        .rounding(4.0),
                );
                let col = if m.club_exterieur_id == mon_club_id { Color32::GOLD } else { Color32::WHITE };
                ui.label(
                    RichText::new(&m.club_exterieur_nom)
                        .font(FontId::proportional(13.5))
                        .strong()
                        .color(col),
                );
                if m.club_exterieur_id == mon_club_id {
                    ui.label(
                        RichText::new(" EXT ")
                            .font(FontId::proportional(9.5))
                            .color(Color32::from_rgb(100, 170, 255)),
                    );
                }
            },
        );
    });
}
