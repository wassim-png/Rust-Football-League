use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2, Pos2, Rect, Align2};
use crate::models::{Joueur, Ecran};

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
        (0.18, 0.35, "MILIEU", "LW"),
        (0.50, 0.33, "MILIEU", "CAM"),
        (0.82, 0.35, "MILIEU", "RW"),
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

    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, panel_rect);

    let (formation_nom, formation) = &FORMATIONS[*formation_idx];

    ui.vertical_centered(|ui| {
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("COMPOSITION — {}", nom_club.to_uppercase()))
                    .font(FontId::proportional(24.0))
                    .strong()
                    .color(Color32::WHITE)
                    .background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 200)),
            );

            ui.add_space(10.0);

            let ancien_idx = *formation_idx;
            egui::ComboBox::from_id_source("choix_formation")
                .selected_text(
                    RichText::new(*formation_nom)
                        .font(FontId::proportional(18.0))
                        .strong()
                        .color(Color32::from_rgb(100, 200, 255)),
                )
                .width(120.0)
                .show_ui(ui, |ui| {
                    for (i, (nom, _)) in FORMATIONS.iter().enumerate() {
                        if ui.selectable_label(i == *formation_idx, *nom).clicked() {
                            *formation_idx = i;
                        }
                    }
                });

            // Reset composition si on change de formation
            if *formation_idx != ancien_idx {
                *composition = std::array::from_fn(|_| None);
                *slot_actif = None;
                *capitaine_slot = None;
            }
        });

        let nb_choisis = composition.iter().filter(|s| s.is_some()).count();
        let couleur = if nb_choisis == 11 {
            Color32::from_rgb(76, 175, 80)
        } else {
            Color32::from_rgb(255, 193, 7)
        };

        ui.label(
            RichText::new(format!("{}/11 postes remplis", nb_choisis))
                .font(FontId::proportional(15.0))
                .color(couleur)
                .strong(),
        );
    });

    let terrain_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 20.0, panel_rect.min.y + 70.0),
        Vec2::new(panel_rect.width() - 40.0, panel_rect.height() - 140.0),
    );

    for (slot_idx, (x_pct, y_pct, poste, label)) in formation.iter().enumerate() {
        let center = Pos2::new(
            terrain_rect.min.x + terrain_rect.width() * x_pct,
            terrain_rect.min.y + terrain_rect.height() * y_pct,
        );
        let radius = 30.0;

        let est_actif = *slot_actif == Some(slot_idx);
        let est_rempli = composition[slot_idx].is_some();

        let (fill, stroke_col) = if est_actif {
            (
                Color32::from_rgba_unmultiplied(255, 255, 255, 60),
                Color32::WHITE,
            )
        } else if est_rempli {
            (
                Color32::from_rgba_unmultiplied(46, 125, 50, 220),
                Color32::from_rgb(76, 175, 80),
            )
        } else {
            (
                Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                couleur_poste(poste),
            )
        };

        ui.painter()
            .circle(center, radius, fill, Stroke::new(2.5, stroke_col));

        if let Some(joueur) = &composition[slot_idx] {
            let nom_court = if joueur.nom.len() > 10 {
                format!("{}.", &joueur.nom[..9])
            } else {
                joueur.nom.clone()
            };

            ui.painter().text(
                center + egui::vec2(0.0, -6.0),
                Align2::CENTER_CENTER,
                format!("{}", joueur.reputation),
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

            // Badge capitaine ©
            if *capitaine_slot == Some(slot_idx) {
                let badge_pos = Pos2::new(center.x + radius * 0.65, center.y - radius * 0.65);
                ui.painter().circle_filled(badge_pos, 10.0, Color32::from_rgb(255, 215, 0));
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
                FontId::proportional(14.0),
                couleur_poste(poste),
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

        egui::SidePanel::right("selection_joueur")
            .min_width(280.0)
            .max_width(320.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(15, 15, 25, 240))
                    .inner_margin(10.0)
                    .stroke(Stroke::new(2.0, couleur_poste(poste_requis))),
            )
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(format!("Choisir — {} ({})", label, poste_requis))
                            .font(FontId::proportional(18.0))
                            .strong()
                            .color(couleur_poste(poste_requis)),
                    );

                    if composition[active_slot].is_some() {
                        ui.add_space(5.0);
                        let btn = egui::Button::new(
                            RichText::new("✖ Retirer le joueur")
                                .font(FontId::proportional(13.0))
                                .color(Color32::WHITE),
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
                                .outer_margin(egui::Margin {
                                    left: 0.0,
                                    right: 0.0,
                                    top: 2.0,
                                    bottom: 2.0,
                                })
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(&joueur.nom)
                                                .font(FontId::proportional(14.0))
                                                .color(Color32::WHITE),
                                        );

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!("{}", joueur.reputation))
                                                        .font(FontId::proportional(14.0))
                                                        .strong()
                                                        .color(Color32::WHITE)
                                                        .background_color(couleur_note(
                                                            joueur.reputation,
                                                        )),
                                                );

                                                // Forme du joueur
                                                if let Some(forme) = joueur.forme {
                                                    let forme_color = if forme >= 80 {
                                                        Color32::from_rgb(76, 175, 80)
                                                    } else if forme >= 50 {
                                                        Color32::from_rgb(255, 193, 7)
                                                    } else {
                                                        Color32::from_rgb(244, 67, 54)
                                                    };
                                                    ui.label(
                                                        RichText::new(format!("⚡ {}%", forme))
                                                            .font(FontId::proportional(12.0))
                                                            .color(forme_color),
                                                    );
                                                }

                                                ui.label(
                                                    RichText::new(format!("{} ans", joueur.age))
                                                        .font(FontId::proportional(12.0))
                                                        .color(Color32::LIGHT_GRAY),
                                                );

                                                // Nationalité avec drapeau
                                                if let Some(nat) = &joueur.nationalite {
                                                    let flag_path = drapeau_pays(nat);
                                                    ui.add(
                                                        egui::Image::new(flag_path)
                                                            .fit_to_exact_size(egui::vec2(20.0, 14.0))
                                                            .rounding(2.0),
                                                    );
                                                }
                                            },
                                        );
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

    let boutons_rect = Rect::from_min_size(
        Pos2::new(panel_rect.min.x + 10.0, panel_rect.max.y - 55.0),
        Vec2::new(panel_rect.width() - 20.0, 50.0),
    );

    ui.allocate_ui_at_rect(boutons_rect, |ui| {
        ui.horizontal(|ui| {
            let btn_retour = egui::Button::new(
                RichText::new("⬅ Retour")
                    .font(FontId::proportional(16.0))
                    .color(Color32::WHITE),
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

            // Bouton capitaine
            let joueurs_places: Vec<(usize, &Joueur)> = composition
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.as_ref().map(|j| (i, j)))
                .collect();

            if !joueurs_places.is_empty() {
                let capitaine_label = if let Some(cap_idx) = *capitaine_slot {
                    if let Some(j) = &composition[cap_idx] {
                        format!("👑 {}", j.nom)
                    } else {
                        "👑 Capitaine".to_string()
                    }
                } else {
                    "👑 Capitaine".to_string()
                };

                egui::ComboBox::from_id_source("choix_capitaine")
                    .selected_text(
                        RichText::new(&capitaine_label)
                            .font(FontId::proportional(14.0))
                            .color(Color32::from_rgb(255, 215, 0)),
                    )
                    .width(180.0)
                    .show_ui(ui, |ui| {
                        for (slot_idx, joueur) in &joueurs_places {
                            let est_capitaine = *capitaine_slot == Some(*slot_idx);
                            let label = if est_capitaine {
                                format!("👑 {}", joueur.nom)
                            } else {
                                joueur.nom.clone()
                            };
                            if ui.selectable_label(est_capitaine, &label).clicked() {
                                *capitaine_slot = Some(*slot_idx);
                            }
                        }
                    });
            }

            ui.add_space(15.0);

            let nb_choisis = composition.iter().filter(|s| s.is_some()).count();
            let peut_valider = nb_choisis == 11;

            let btn_couleur = if peut_valider {
                Color32::from_rgb(46, 125, 50)
            } else {
                Color32::from_gray(50)
            };

            let txt_couleur = if peut_valider {
                Color32::WHITE
            } else {
                Color32::from_gray(100)
            };

            let btn_valider = egui::Button::new(
                RichText::new("✅ Valider la composition")
                    .font(FontId::proportional(16.0))
                    .color(txt_couleur),
            )
            .fill(btn_couleur)
            .stroke(Stroke::new(
                1.0,
                if peut_valider {
                    Color32::from_rgb(76, 175, 80)
                } else {
                    Color32::from_gray(70)
                },
            ))
            .rounding(8.0)
            .min_size(Vec2::new(220.0, 40.0));

            let response = ui.add(btn_valider);
            if peut_valider && response.clicked() {
                *slot_actif = None;
                composition_validee = true;
                println!("Composition validée !");
            }
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