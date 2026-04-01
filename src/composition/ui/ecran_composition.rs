use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2, Pos2, Rect, Align2};
use crate::models::{Joueur, Ecran};

const OR: Color32 = Color32::from_rgb(212, 175, 55);
const FOND_HEADER: Color32 = Color32::from_rgb(10, 12, 22);
const VERT_BTN: Color32 = Color32::from_rgb(46, 125, 50);
const ROUGE_BTN: Color32 = Color32::from_rgb(198, 40, 40);

const FORMATIONS: [(&str, [(f32, f32, &str, &str); 11]); 6] = [
    // 4-3-3
    ("4-3-3", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.15, 0.70, "DEFENSE", "LB"),
        (0.38, 0.73, "DEFENSE", "CB"),
        (0.62, 0.73, "DEFENSE", "CB"),
        (0.85, 0.70, "DEFENSE", "RB"),
        (0.25, 0.48, "MILIEU", "CM"),
        (0.50, 0.45, "MILIEU", "CM"),
        (0.75, 0.48, "MILIEU", "CM"),
        (0.18, 0.22, "ATTAQUE", "LW"),
        (0.50, 0.18, "ATTAQUE", "ST"),
        (0.82, 0.22, "ATTAQUE", "RW"),
    ]),
    // 4-4-2
    ("4-4-2", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.15, 0.70, "DEFENSE", "LB"),
        (0.38, 0.73, "DEFENSE", "CB"),
        (0.62, 0.73, "DEFENSE", "CB"),
        (0.85, 0.70, "DEFENSE", "RB"),
        (0.15, 0.48, "MILIEU", "LM"),
        (0.38, 0.45, "MILIEU", "CM"),
        (0.62, 0.45, "MILIEU", "CM"),
        (0.85, 0.48, "MILIEU", "RM"),
        (0.35, 0.20, "ATTAQUE", "ST"),
        (0.65, 0.20, "ATTAQUE", "ST"),
    ]),
    // 3-5-2
    ("3-5-2", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.25, 0.72, "DEFENSE", "CB"),
        (0.50, 0.75, "DEFENSE", "CB"),
        (0.75, 0.72, "DEFENSE", "CB"),
        (0.10, 0.50, "MILIEU", "LWB"),
        (0.35, 0.48, "MILIEU", "CM"),
        (0.50, 0.43, "MILIEU", "CM"),
        (0.65, 0.48, "MILIEU", "CM"),
        (0.90, 0.50, "MILIEU", "RWB"),
        (0.35, 0.20, "ATTAQUE", "ST"),
        (0.65, 0.20, "ATTAQUE", "ST"),
    ]),
    // 4-2-3-1
    ("4-2-3-1", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.15, 0.70, "DEFENSE", "LB"),
        (0.38, 0.73, "DEFENSE", "CB"),
        (0.62, 0.73, "DEFENSE", "CB"),
        (0.85, 0.70, "DEFENSE", "RB"),
        (0.35, 0.53, "MILIEU", "CDM"),
        (0.65, 0.53, "MILIEU", "CDM"),
        (0.18, 0.35, "ATTAQUE", "LW"),
        (0.50, 0.33, "ATTAQUE", "CAM"),
        (0.82, 0.35, "ATTAQUE", "RW"),
        (0.50, 0.18, "ATTAQUE", "ST"),
    ]),
    // 4-5-1
    ("4-5-1", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.15, 0.70, "DEFENSE", "LB"),
        (0.38, 0.73, "DEFENSE", "CB"),
        (0.62, 0.73, "DEFENSE", "CB"),
        (0.85, 0.70, "DEFENSE", "RB"),
        (0.12, 0.48, "MILIEU", "LM"),
        (0.35, 0.45, "MILIEU", "CM"),
        (0.50, 0.42, "MILIEU", "CM"),
        (0.65, 0.45, "MILIEU", "CM"),
        (0.88, 0.48, "MILIEU", "RM"),
        (0.50, 0.18, "ATTAQUE", "ST"),
    ]),
    // 3-4-3
    ("3-4-3", [
        (0.50, 0.88, "GARDIEN", "GK"),
        (0.25, 0.72, "DEFENSE", "CB"),
        (0.50, 0.75, "DEFENSE", "CB"),
        (0.75, 0.72, "DEFENSE", "CB"),
        (0.15, 0.48, "MILIEU", "LM"),
        (0.38, 0.45, "MILIEU", "CM"),
        (0.62, 0.45, "MILIEU", "CM"),
        (0.85, 0.48, "MILIEU", "RM"),
        (0.18, 0.22, "ATTAQUE", "LW"),
        (0.50, 0.18, "ATTAQUE", "ST"),
        (0.82, 0.22, "ATTAQUE", "RW"),
    ]),
];

fn couleur_poste(poste: &str) -> Color32 {
    match poste {
        "GARDIEN" => Color32::from_rgb(255, 193, 7),
        "DEFENSE" => Color32::from_rgb(66, 165, 245),
        "MILIEU" => Color32::from_rgb(102, 187, 106),
        "ATTAQUE" => Color32::from_rgb(239, 83, 80),
        _ => Color32::GRAY,
    }
}

fn couleur_note(note: i32) -> Color32 {
    match note {
        80..=100 => Color32::from_rgb(76, 175, 80),
        65..=79 => Color32::from_rgb(139, 195, 74),
        50..=64 => Color32::from_rgb(255, 193, 7),
        _ => Color32::from_rgb(244, 67, 54),
    }
}

pub fn render(
    ui: &mut Ui,
    joueurs: &[Joueur],
    composition: &mut [Option<Joueur>; 11],
    slot_actif: &mut Option<usize>,
    capitaine_slot: &mut Option<usize>,
    formation_idx: &mut usize,
    ecran_actuel: &mut Ecran,
    nom_club: &str,
)->bool {
    let mut composition_validee = false;
    let panel_rect = ui.max_rect();

    egui::Image::new("file://assets/compo3.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, panel_rect);

    let (formation_nom, formation) = &FORMATIONS[*formation_idx];

    let nb_choisis = composition.iter().filter(|s| s.is_some()).count();
    let somme_notes: i32 = composition
        .iter()
        .filter_map(|s| s.as_ref().and_then(|j| j.note_actuelle))
        .sum();
    let note_moyenne = if nb_choisis > 0 { somme_notes / nb_choisis as i32 } else { 0 };

    // ── Header ──────────────────────────────────────────────────────────
    let header_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 10.0, panel_rect.min.y + 8.0),
        Vec2::new(panel_rect.width() - 20.0, 56.0),
    );
    ui.allocate_ui_at_rect(header_rect, |ui| {
        Frame::none()
            .fill(Color32::from_rgba_unmultiplied(FOND_HEADER.r(), FOND_HEADER.g(), FOND_HEADER.b(), 235))
            .stroke(Stroke::new(1.5, OR))
            .rounding(10.0)
            .inner_margin(egui::Margin { left: 14.0, right: 14.0, top: 8.0, bottom: 8.0 })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Titre
                    ui.label(
                        RichText::new(format!("COMPOSITION — {}", nom_club.to_uppercase()))
                            .font(FontId::proportional(19.0))
                            .color(OR)
                            .strong(),
                    );
                    ui.add_space(14.0);

                    // Sélecteur formation
                    let ancien_idx = *formation_idx;
                    egui::ComboBox::from_id_source("choix_formation")
                        .selected_text(
                            RichText::new(*formation_nom)
                                .font(FontId::proportional(15.0))
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .width(90.0)
                        .show_ui(ui, |ui| {
                            for (i, (nom, _)) in FORMATIONS.iter().enumerate() {
                                if ui.selectable_label(i == *formation_idx, *nom).clicked() {
                                    *formation_idx = i;
                                }
                            }
                        });
                    if *formation_idx != ancien_idx {
                        *composition = std::array::from_fn(|_| None);
                        *slot_actif = None;
                        *capitaine_slot = None;
                    }

                    // Pills droite
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Note moyenne
                        if note_moyenne > 0 {
                            let col_note = couleur_note(note_moyenne);
                            Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(30, 30, 50, 220))
                                .rounding(20.0)
                                .stroke(Stroke::new(1.0, col_note))
                                .inner_margin(egui::Margin { left: 8.0, right: 8.0, top: 3.0, bottom: 3.0 })
                                .show(ui, |ui| {
                                    ui.label(
                                        RichText::new(format!("⭐ {}", note_moyenne))
                                            .font(FontId::proportional(13.0))
                                            .color(col_note)
                                            .strong(),
                                    );
                                });
                            ui.add_space(8.0);
                        }
                        // Postes remplis
                        let col_postes = if nb_choisis == 11 {
                            Color32::from_rgb(76, 175, 80)
                        } else {
                            OR
                        };
                        Frame::none()
                            .fill(Color32::from_rgba_unmultiplied(30, 30, 50, 220))
                            .rounding(20.0)
                            .stroke(Stroke::new(1.0, col_postes))
                            .inner_margin(egui::Margin { left: 8.0, right: 8.0, top: 3.0, bottom: 3.0 })
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(format!("👥 {}/11", nb_choisis))
                                        .font(FontId::proportional(13.0))
                                        .color(col_postes)
                                        .strong(),
                                );
                            });
                    });
                });
            });
    });

    let terrain_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 20.0, panel_rect.min.y + 74.0),
        Vec2::new(panel_rect.width() - 40.0, panel_rect.height() - 74.0 - 66.0),
    );

    for (slot_idx, (x_pct, y_pct, poste, label)) in formation.iter().enumerate() {
        let center = Pos2::new(
            terrain_rect.min.x + terrain_rect.width() * x_pct,
            terrain_rect.min.y + terrain_rect.height() * y_pct,
        );
        let radius = 30.0;

        let est_actif = *slot_actif == Some(slot_idx);
        let est_rempli = composition[slot_idx].is_some();

        let col_poste = couleur_poste(poste);

        // Ombre portée
        ui.painter().circle_filled(
            center + egui::vec2(2.0, 2.5),
            radius + 1.0,
            Color32::from_rgba_unmultiplied(0, 0, 0, 110),
        );

        let (fill, stroke_col, stroke_w) = if est_actif {
            (Color32::from_rgba_unmultiplied(255, 255, 255, 55), Color32::WHITE, 3.0)
        } else if est_rempli {
            (Color32::from_rgba_unmultiplied(col_poste.r(), col_poste.g(), col_poste.b(), 50), col_poste, 2.5)
        } else {
            (Color32::from_rgba_unmultiplied(8, 8, 18, 185), col_poste, 2.0)
        };

        ui.painter().circle(center, radius, fill, Stroke::new(stroke_w, stroke_col));

        if let Some(joueur) = &composition[slot_idx] {
            let note = joueur.note_actuelle.unwrap_or(joueur.reputation);
            let col_note = couleur_note(note);

            // Note en gros dans le cercle
            ui.painter().text(
                center + egui::vec2(0.0, -4.0),
                Align2::CENTER_CENTER,
                format!("{}", note),
                FontId::proportional(14.0),
                col_note,
            );

            // Nom en pill semi-transparente sous le cercle
            let nom = &joueur.nom;
            let pill_w = (nom.len() as f32 * 7.5).max(70.0).min(120.0);
            let pill_center = Pos2::new(center.x, center.y + radius + 10.0);
            ui.painter().rect_filled(
                Rect::from_center_size(pill_center, Vec2::new(pill_w, 17.0)),
                4.0,
                Color32::from_rgba_unmultiplied(0, 0, 0, 185),
            );
            ui.painter().text(
                pill_center,
                Align2::CENTER_CENTER,
                nom.as_str(),
                FontId::proportional(12.0),
                Color32::WHITE,
            );

            // Badge capitaine
            if *capitaine_slot == Some(slot_idx) {
                let badge_pos = Pos2::new(center.x + radius * 0.65, center.y - radius * 0.65);
                ui.painter().circle_filled(badge_pos, 10.0, OR);
                ui.painter().text(
                    badge_pos,
                    Align2::CENTER_CENTER,
                    "C",
                    FontId::proportional(11.0),
                    Color32::BLACK,
                );
            }
        } else {
            ui.painter().text(
                center,
                Align2::CENTER_CENTER,
                *label,
                FontId::proportional(13.0),
                col_poste,
            );
        }

        let slot_rect = Rect::from_center_size(center, Vec2::splat(radius * 2.0));
        let response = ui.allocate_rect(slot_rect, egui::Sense::click());

        if response.clicked() {
            if est_actif {
                *slot_actif = None;
            } else {
                *slot_actif = Some(slot_idx);
            }
        }
    }

    if let Some(active_slot) = *slot_actif {
        let (_, _, poste_requis, label) = formation[active_slot];

        let deja_pris_ids: Vec<i32> = composition
            .iter()
            .enumerate()
            .filter(|(i, s)| s.is_some() && *i != active_slot)
            .filter_map(|(_, s)| s.as_ref())
            .filter_map(|j| Some(j.id))
            .collect();

        let joueurs_disponibles: Vec<&Joueur> = joueurs
            .iter()
            .filter(|j| j.poste == poste_requis && !deja_pris_ids.contains(&j.id))
            .collect();

        let col_poste = couleur_poste(poste_requis);

        egui::SidePanel::right("selection_joueur")
            .min_width(285.0)
            .max_width(320.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(12, 14, 24, 248))
                    .inner_margin(0.0)
                    .stroke(Stroke::new(2.0, col_poste)),
            )
            .show_inside(ui, |ui| {
                // Header coloré par poste
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(col_poste.r(), col_poste.g(), col_poste.b(), 28))
                    .inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 10.0, bottom: 10.0 })
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("{} — {}", label, poste_requis))
                                    .font(FontId::proportional(17.0))
                                    .strong()
                                    .color(col_poste),
                            );

                            if composition[active_slot].is_some() {
                                ui.add_space(6.0);
                                let btn = egui::Button::new(
                                    RichText::new("✖  Retirer le joueur")
                                        .font(FontId::proportional(13.0))
                                        .color(Color32::WHITE),
                                )
                                .fill(ROUGE_BTN)
                                .stroke(Stroke::NONE)
                                .rounding(6.0);
                                if ui.add(btn).clicked() {
                                    composition[active_slot] = None;
                                    *slot_actif = None;
                                }
                            }
                        });
                    });

                ui.add_space(4.0);
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 10.0)
                    .show(ui, |ui| {
                        if joueurs_disponibles.is_empty() {
                            ui.label(
                                RichText::new("Aucun joueur disponible")
                                    .color(Color32::GRAY),
                            );
                        }

                        for joueur in &joueurs_disponibles {
                            let joueur_id = joueur.id;
                            let est_actuel = composition[active_slot]
                                .as_ref()
                                .and_then(|j| Some(j.id))
                                == Some(joueur_id);

                            let (bg, border) = if est_actuel {
                                (
                                    Color32::from_rgba_unmultiplied(col_poste.r(), col_poste.g(), col_poste.b(), 45),
                                    col_poste,
                                )
                            } else {
                                (
                                    Color32::from_rgba_unmultiplied(28, 28, 42, 230),
                                    Color32::from_gray(55),
                                )
                            };

                            Frame::none()
                                .fill(bg)
                                .rounding(7.0)
                                .stroke(Stroke::new(1.0, border))
                                .inner_margin(egui::Margin { left: 10.0, right: 8.0, top: 7.0, bottom: 7.0 })
                                .outer_margin(egui::Margin { left: 6.0, right: 6.0, top: 2.0, bottom: 2.0 })
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Nom + infos secondaires
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new(&joueur.nom)
                                                    .font(FontId::proportional(14.0))
                                                    .color(Color32::WHITE),
                                            );
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new(format!("{} ans", joueur.age))
                                                        .font(FontId::proportional(11.0))
                                                        .color(Color32::GRAY),
                                                );
                                                if let Some(nat) = &joueur.nationalite {
                                                    let flag_path = drapeau_pays(nat);
                                                    ui.add(
                                                        egui::Image::new(flag_path)
                                                            .fit_to_exact_size(egui::vec2(18.0, 13.0))
                                                            .rounding(2.0),
                                                    );
                                                }
                                            });
                                        });

                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            // Badge note coloré
                                            let note = joueur.note_actuelle.unwrap_or(joueur.reputation);
                                            let col_n = couleur_note(note);
                                            Frame::none()
                                                .fill(col_n)
                                                .rounding(5.0)
                                                .inner_margin(egui::Margin { left: 6.0, right: 6.0, top: 2.0, bottom: 2.0 })
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(format!("{}", note))
                                                            .font(FontId::proportional(13.0))
                                                            .strong()
                                                            .color(Color32::WHITE),
                                                    );
                                                });

                                            // Forme
                                            if let Some(forme) = joueur.forme {
                                                let forme_color = if forme >= 80 {
                                                    Color32::from_rgb(76, 175, 80)
                                                } else if forme >= 50 {
                                                    Color32::from_rgb(255, 193, 7)
                                                } else {
                                                    Color32::from_rgb(244, 67, 54)
                                                };
                                                ui.label(
                                                    RichText::new(format!("⚡{}%", forme))
                                                        .font(FontId::proportional(11.0))
                                                        .color(forme_color),
                                                );
                                            }
                                        });
                                    });

                                    let response = ui.interact(
                                        ui.min_rect(),
                                        ui.id().with(format!("joueur_{}", joueur_id)),
                                        egui::Sense::click(),
                                    );

                                    if response.clicked() {
                                        composition[active_slot] = Some((*joueur).clone());
                                        *slot_actif = None;
                                    }
                                });
                        }
                    });
            });
    }

    let bar_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 10.0, panel_rect.max.y - 58.0),
        Vec2::new(panel_rect.width() - 20.0, 52.0),
    );

    ui.allocate_ui_at_rect(bar_rect, |ui| {
        Frame::none()
            .fill(Color32::from_rgba_unmultiplied(FOND_HEADER.r(), FOND_HEADER.g(), FOND_HEADER.b(), 235))
            .stroke(Stroke::new(1.5, OR))
            .rounding(10.0)
            .inner_margin(egui::Margin { left: 12.0, right: 12.0, top: 8.0, bottom: 8.0 })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Retour
                    let btn_retour = egui::Button::new(
                        RichText::new("⬅  Retour")
                            .font(FontId::proportional(14.0))
                            .color(Color32::LIGHT_GRAY),
                    )
                    .fill(Color32::from_rgb(35, 35, 55))
                    .stroke(Stroke::new(1.0, Color32::from_gray(70)))
                    .rounding(7.0)
                    .min_size(Vec2::new(110.0, 34.0));
                    if ui.add(btn_retour).clicked() {
                        *slot_actif = None;
                        *ecran_actuel = Ecran::MenuPrincipal;
                    }

                    ui.add_space(12.0);

                    // ComboBox capitaine
                    let joueurs_places: Vec<(usize, &Joueur)> = composition
                        .iter()
                        .enumerate()
                        .filter_map(|(i, s)| s.as_ref().map(|j| (i, j)))
                        .collect();

                    if !joueurs_places.is_empty() {
                        let capitaine_label = if let Some(cap_idx) = *capitaine_slot {
                            if let Some(j) = &composition[cap_idx] {
                                format!("👑  {}", j.nom)
                            } else {
                                "👑  Capitaine".to_string()
                            }
                        } else {
                            "👑  Choisir capitaine".to_string()
                        };

                        egui::ComboBox::from_id_source("choix_capitaine")
                            .selected_text(
                                RichText::new(&capitaine_label)
                                    .font(FontId::proportional(14.0))
                                    .color(OR),
                            )
                            .width(200.0)
                            .show_ui(ui, |ui| {
                                for (slot_idx, joueur) in &joueurs_places {
                                    let est_capitaine = *capitaine_slot == Some(*slot_idx);
                                    let label = if est_capitaine {
                                        format!("👑  {}", joueur.nom)
                                    } else {
                                        joueur.nom.clone()
                                    };
                                    if ui.selectable_label(est_capitaine, &label).clicked() {
                                        *capitaine_slot = Some(*slot_idx);
                                    }
                                }
                            });
                    }

                    // Valider à droite
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let peut_valider = nb_choisis == 11 && *capitaine_slot != None;

                        let (fill_v, txt_v, stroke_v) = if peut_valider {
                            (VERT_BTN, Color32::WHITE, Color32::from_rgb(76, 175, 80))
                        } else {
                            (Color32::from_rgb(38, 38, 55), Color32::from_gray(85), Color32::from_gray(60))
                        };

                        let btn_valider = egui::Button::new(
                            RichText::new("✅  Valider la composition")
                                .font(FontId::proportional(14.0))
                                .color(txt_v),
                        )
                        .fill(fill_v)
                        .stroke(Stroke::new(1.0, stroke_v))
                        .rounding(7.0)
                        .min_size(Vec2::new(220.0, 34.0));

                        let response = ui.add_enabled(peut_valider, btn_valider);
                        if peut_valider && response.clicked() {
                            *slot_actif = None;
                            composition_validee = true;
                            println!("Composition validée !");
                        }

                        // Hint si pas prêt
                        if !peut_valider {
                            ui.add_space(10.0);
                            let hint = if nb_choisis < 11 {
                                format!("{} poste{} manquant{}",
                                    11 - nb_choisis,
                                    if 11 - nb_choisis > 1 { "s" } else { "" },
                                    if 11 - nb_choisis > 1 { "s" } else { "" })
                            } else {
                                "Désignez un capitaine".to_string()
                            };
                            ui.label(
                                RichText::new(hint)
                                    .font(FontId::proportional(12.0))
                                    .color(Color32::from_rgb(180, 140, 50)),
                            );
                        }
                    });
                });
            });
    });

    return composition_validee
}

fn drapeau_pays(pays: &str) -> String {
    let fichier = match pays {
        "France" => "france",
        "Bresil" => "bresil",
        "Portugal" => "portugal",
        "Maroc" => "maroc",
        "Espagne" => "espagne",
        "Angleterre" => "angleterre",
        "Allemagne" => "allemagne",
        "Argentine" => "argentine",
        "Senegal" => "senegal",
        "Pays-Bas" => "pays-bas",
        "Belgique" => "belgique",
        "Cote d'Ivoire" => "cote_divoire",
        "Algerie" => "algerie",
        "Cameroun" => "cameroun",
        "Canada" => "canada",
        "Suisse" => "suisse",
        "Danemark" => "danemark",
        "Croatie" => "croatie",
        "Colombie" => "colombie",
        "Norvege" => "norvege",
        "Pologne" => "pologne",
        "Suede" => "suede",
        "Tunisie" => "tunisie",
        "Egypte" => "egypte",
        "Ghana" => "ghana",
        "Nigeria" => "nigeria",
        "Mali" => "mali",
        "Guinee" => "guinee",
        "Russie" => "russie",
        "Japon" => "japon",
        "Coree du Sud" => "coree_du_sud",
        "Etats-Unis" => "etats-unis",
        "Uruguay" => "uruguay",
        "Equateur" => "equateur",
        "Chili" => "chili",
        "Slovaquie" => "slovaquie",
        "Slovenie" => "slovenie",
        "Autriche" => "autriche",
        "Serbie" => "serbie",
        "Turquie" => "turquie",
        "Georgie" => "georgie",
        "Kosovo" => "kosovo",
        "Roumanie" => "roumanie",
        "Hongrie" => "hongrie",
        "Finlande" => "finlande",
        "Angola" => "angola",
        "RD Congo" => "rd_congo",
        "Centrafrique" => "centrafrique",
        "Burundi" => "burundi",
        "Gabon" => "gabon",
        "Benin" => "benin",
        "Gambie" => "gambie",
        "Madagascar" => "madagascar",
        "Zimbabwe" => "zimbabwe",
        "Haiti" => "haiti",
        "Panama" => "panama",
        "Venezuela" => "venezuela",
        "Bosnie" => "bosnie",
        "Australie" => "australie",
        "Guinee-Bissau" => "guinee-bissau",
        "Ouzbekistan" => "ouzbekistan",
        "Pays de Galles" => "pays_de_galles",
        _ => "france",
    };
    format!("file://assets/flags/{}.png", fichier)
}