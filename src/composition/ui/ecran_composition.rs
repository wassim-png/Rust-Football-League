use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2, Pos2, Rect, Align2};
use crate::models::{Joueur, Ecran};

/// Les 11 postes du 4-3-3 avec leurs positions visuelles (en % du terrain)
const FORMATION_433: [(f32, f32, &str, &str); 11] = [
    // (x%, y%, poste_db, label_affiche)
    (0.50, 0.88, "GARDIEN", "GK"),    // Gardien

    (0.15, 0.70, "DEFENSE", "LB"),    // Arrière Gauche
    (0.38, 0.73, "DEFENSE", "CB"),    // Défenseur Central G
    (0.62, 0.73, "DEFENSE", "CB"),    // Défenseur Central D
    (0.85, 0.70, "DEFENSE", "RB"),    // Arrière Droit

    (0.25, 0.48, "MILIEU", "CM"),     // Milieu Gauche
    (0.50, 0.45, "MILIEU", "CM"),     // Milieu Central
    (0.75, 0.48, "MILIEU", "CM"),     // Milieu Droit

    (0.18, 0.22, "ATTAQUE", "LW"),    // Ailier Gauche
    (0.50, 0.18, "ATTAQUE", "ST"),    // Avant-Centre
    (0.82, 0.22, "ATTAQUE", "RW"),    // Ailier Droit
];

/// Couleur associée à chaque poste
fn couleur_poste(poste: &str) -> Color32 {
    match poste {
        "GARDIEN"  => Color32::from_rgb(255, 193, 7),
        "DEFENSE"  => Color32::from_rgb(66, 165, 245),
        "MILIEU"   => Color32::from_rgb(102, 187, 106),
        "ATTAQUE"  => Color32::from_rgb(239, 83, 80),
        _          => Color32::GRAY,
    }
}

fn couleur_note(note: i32) -> Color32 {
    match note {
        80..=100 => Color32::from_rgb(76, 175, 80),
        65..=79  => Color32::from_rgb(139, 195, 74),
        50..=64  => Color32::from_rgb(255, 193, 7),
        _        => Color32::from_rgb(244, 67, 54),
    }
}

pub fn render(
    ui: &mut Ui,
    joueurs: &[Joueur],
    composition: &mut [Option<usize>; 11], // index dans joueurs pour chaque slot
    slot_actif: &mut Option<usize>,         // slot en cours de sélection
    ecran_actuel: &mut Ecran,
    nom_club: &str,
) {
    let panel_rect = ui.max_rect();

    // === Fond pelouse ===
    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, panel_rect);

    // === EN-TÊTE ===
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.label(
            RichText::new(format!("COMPOSITION 4-3-3 — {}", nom_club.to_uppercase()))
                .font(FontId::proportional(28.0))
                .strong()
                .color(Color32::WHITE)
                .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 200))
        );

        let nb_choisis = composition.iter().filter(|s| s.is_some()).count();
        let couleur = if nb_choisis == 11 { Color32::from_rgb(76, 175, 80) } else { Color32::from_rgb(255, 193, 7) };
        ui.label(
            RichText::new(format!("{}/11 postes remplis", nb_choisis))
                .font(FontId::proportional(15.0))
                .color(couleur)
                .strong()
        );
    });

    // === Zone du terrain pour les slots ===
    // On utilise une zone carrée dans le panel pour placer les cercles
    let terrain_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 20.0, panel_rect.min.y + 70.0),
        Vec2::new(panel_rect.width() - 40.0, panel_rect.height() - 140.0),
    );

    // === Dessiner les 11 slots ===
    for (slot_idx, (x_pct, y_pct, _poste, label)) in FORMATION_433.iter().enumerate() {
        let center = Pos2::new(
            terrain_rect.min.x + terrain_rect.width() * x_pct,
            terrain_rect.min.y + terrain_rect.height() * y_pct,
        );
        let radius = 30.0;

        let est_actif = *slot_actif == Some(slot_idx);
        let est_rempli = composition[slot_idx].is_some();

        // Couleur du cercle
        let (fill, stroke_col) = if est_actif {
            (Color32::from_rgba_unmultiplied(255, 255, 255, 60), Color32::WHITE)
        } else if est_rempli {
            (Color32::from_rgba_unmultiplied(46, 125, 50, 220), Color32::from_rgb(76, 175, 80))
        } else {
            (Color32::from_rgba_unmultiplied(0, 0, 0, 180), couleur_poste(_poste))
        };

        // Dessiner le cercle
        ui.painter().circle(center, radius, fill, Stroke::new(2.5, stroke_col));

        // Texte dans le cercle
        if let Some(joueur_idx) = composition[slot_idx] {
            // Nom court du joueur
            let joueur = &joueurs[joueur_idx];
            let nom_court = if joueur.nom.len() > 10 {
                format!("{}.", &joueur.nom[..9])
            } else {
                joueur.nom.clone()
            };
            ui.painter().text(
                center + egui::vec2(0.0, -6.0),
                Align2::CENTER_CENTER,
                format!("#{}", joueur.numero),
                FontId::proportional(11.0),
                Color32::from_rgb(200, 200, 200),
            );
            ui.painter().text(
                center + egui::vec2(0.0, 8.0),
                Align2::CENTER_CENTER,
                nom_court,
                FontId::proportional(10.0),
                Color32::WHITE,
            );
        } else {
            // Label du poste
            ui.painter().text(
                center,
                Align2::CENTER_CENTER,
                *label,
                FontId::proportional(14.0),
                couleur_poste(_poste),
            );
        }

        // Détection du clic sur le slot
        let slot_rect = Rect::from_center_size(center, Vec2::splat(radius * 2.0));
        let response = ui.allocate_rect(slot_rect, egui::Sense::click());
        if response.clicked() {
            if est_actif {
                *slot_actif = None; // fermer le popup
            } else {
                *slot_actif = Some(slot_idx);
            }
        }
    }

    // === POPUP DE SÉLECTION DE JOUEUR ===
    if let Some(active_slot) = *slot_actif {
        let (_, _, poste_requis, label) = FORMATION_433[active_slot];

        // Joueurs déjà sélectionnés (sauf le slot actif)
        let deja_pris: Vec<usize> = composition.iter().enumerate()
            .filter(|(i, s)| s.is_some() && *i != active_slot)
            .map(|(_, s)| s.unwrap())
            .collect();

        let joueurs_disponibles: Vec<(usize, &Joueur)> = joueurs.iter().enumerate()
            .filter(|(idx, j)| j.poste == poste_requis && !deja_pris.contains(idx))
            .collect();

        // Panneau latéral droit pour la sélection
        egui::SidePanel::right("selection_joueur")
            .min_width(280.0)
            .max_width(320.0)
            .frame(Frame::none()
                .fill(Color32::from_rgba_unmultiplied(15, 15, 25, 240))
                .inner_margin(10.0)
                .stroke(Stroke::new(2.0, couleur_poste(poste_requis)))
            )
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(format!("Choisir — {} ({})", label, poste_requis))
                            .font(FontId::proportional(18.0))
                            .strong()
                            .color(couleur_poste(poste_requis))
                    );

                    // Bouton pour vider le slot
                    if composition[active_slot].is_some() {
                        ui.add_space(5.0);
                        let btn = egui::Button::new(
                            RichText::new("✖ Retirer le joueur")
                                .font(FontId::proportional(13.0))
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgb(198, 40, 40))
                        .rounding(4.0);
                        if ui.add(btn).clicked() {
                            composition[active_slot] = None;
                            *slot_actif = None;
                        }
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                egui::ScrollArea::vertical().max_height(ui.available_height() - 10.0).show(ui, |ui| {
                    if joueurs_disponibles.is_empty() {
                        ui.label(
                            RichText::new("Aucun joueur disponible")
                                .color(Color32::GRAY)
                        );
                    }

                    for (idx, joueur) in &joueurs_disponibles {
                        let est_actuel = composition[active_slot] == Some(*idx);
                        let bg = if est_actuel {
                            Color32::from_rgba_unmultiplied(46, 125, 50, 180)
                        } else {
                            Color32::from_rgba_unmultiplied(40, 40, 50, 200)
                        };

                        Frame::none()
                            .fill(bg)
                            .rounding(6.0)
                            .stroke(Stroke::new(1.0, Color32::from_gray(60)))
                            .inner_margin(6.0)
                            .outer_margin(egui::Margin { left: 0.0, right: 0.0, top: 2.0, bottom: 2.0 })
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Numéro
                                    ui.label(
                                        RichText::new(format!("#{}", joueur.numero))
                                            .font(FontId::proportional(14.0))
                                            .strong()
                                            .color(Color32::WHITE)
                                    );

                                    // Nom
                                    ui.label(
                                        RichText::new(&joueur.nom)
                                            .font(FontId::proportional(14.0))
                                            .color(Color32::WHITE)
                                    );

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Note
                                        ui.label(
                                            RichText::new(format!("{}", joueur.note_actuelle))
                                                .font(FontId::proportional(14.0))
                                                .strong()
                                                .color(Color32::WHITE)
                                                .background_color(couleur_note(joueur.note_actuelle))
                                        );

                                        // Forme
                                        let forme_emoji = if joueur.forme >= 80 { "🟢" } else if joueur.forme >= 50 { "🟡" } else { "🔴" };
                                        ui.label(
                                            RichText::new(format!("{}", forme_emoji))
                                                .font(FontId::proportional(12.0))
                                        );
                                    });
                                });

                                // Clic pour sélectionner
                                let response = ui.interact(
                                    ui.min_rect(),
                                    ui.id().with(format!("joueur_{}", idx)),
                                    egui::Sense::click()
                                );
                                if response.clicked() {
                                    composition[active_slot] = Some(*idx);
                                    *slot_actif = None;
                                }
                            });
                    }
                });
            });
    }

    // === BOUTONS DU BAS ===
    let boutons_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 10.0, panel_rect.max.y - 55.0),
        Vec2::new(panel_rect.width() - 20.0, 50.0),
    );
    ui.allocate_ui_at_rect(boutons_rect, |ui| {
        ui.horizontal(|ui| {
            let btn_retour = egui::Button::new(
                RichText::new("⬅ Retour")
                    .font(FontId::proportional(16.0))
                    .color(Color32::WHITE)
            )
            .fill(Color32::from_rgba_unmultiplied(40, 40, 40, 220))
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .rounding(8.0)
            .min_size(Vec2::new(120.0, 40.0));

            if ui.add(btn_retour).clicked() {
                *slot_actif = None;
                *ecran_actuel = Ecran::MenuPrincipal;
            }

            ui.add_space(15.0);

            let nb_choisis = composition.iter().filter(|s| s.is_some()).count();
            let peut_valider = nb_choisis == 11;
            let btn_couleur = if peut_valider { Color32::from_rgb(46, 125, 50) } else { Color32::from_gray(50) };
            let txt_couleur = if peut_valider { Color32::WHITE } else { Color32::from_gray(100) };

            let btn_valider = egui::Button::new(
                RichText::new("✅ Valider la composition")
                    .font(FontId::proportional(16.0))
                    .color(txt_couleur)
            )
            .fill(btn_couleur)
            .stroke(Stroke::new(1.0, if peut_valider { Color32::from_rgb(76, 175, 80) } else { Color32::from_gray(70) }))
            .rounding(8.0)
            .min_size(Vec2::new(220.0, 40.0));

            let response = ui.add(btn_valider);
            if peut_valider && response.clicked() {
                *slot_actif = None;
                println!("Composition validée !");
            }
        });
    });
}
