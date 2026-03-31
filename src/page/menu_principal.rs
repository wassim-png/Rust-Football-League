use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke};
use crate::models::{Club, Ecran, Match};

pub fn render(ui: &mut Ui, club: &Club, ecran_actuel: &mut Ecran, next_game: &Option<Match>, classement: &Vec<Club>, journee_actuelle: i32) {
    let rect_ecran = ui.max_rect();

    // --- Fond d'écran ---
    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, ui.max_rect());
    ui.painter().rect_filled(
        rect_ecran,
        0.0,
        Color32::from_rgba_unmultiplied(12, 12, 22, 195),
    );

    // --- Header absolu en haut ---
    let header_rect = egui::Rect::from_min_size(
        rect_ecran.min,
        Vec2::new(rect_ecran.width(), 90.0),
    );
    ui.allocate_ui_at_rect(header_rect, |ui| {
        render_header(ui, club, classement, journee_actuelle);
    });
    // allocate_ui_at_rect avance le curseur, on ajoute un gap
    ui.add_space(10.0);

    // ─────────────────────────────────────────────
    //  Section du milieu : 2 colonnes (hauteur fixée
    //  pour laisser la place aux cartes nav en bas)
    // ─────────────────────────────────────────────
    let mut action: Option<Ecran> = None;

    let nav_height = 112.0 + 12.0 + 12.0; // cartes + espacement haut/bas
    let mid_height = (ui.available_height() - nav_height).min(400.0).max(180.0);

    let mid_rect = egui::Rect::from_min_size(
        ui.cursor().min,
        Vec2::new(ui.available_width(), mid_height),
    );
    ui.allocate_ui_at_rect(mid_rect, |ui| {
        ui.columns(2, |cols| {
            if render_prochain_match_card(&mut cols[0], club, next_game, classement, mid_height, journee_actuelle) {
                action = Some(Ecran::ProchainMatch);
            }
            if render_classement_card(&mut cols[1], classement, journee_actuelle, mid_height) {
                action = Some(Ecran::Classement);
            }
        });
    });

    ui.add_space(12.0);

    // ─────────────────────────────────────────────
    //  5 cartes de navigation en bas
    // ─────────────────────────────────────────────
    ui.columns(4, |cols| {
        let nav = [
            ("🏢", "Infos Club",  "Stade & finances",   Ecran::InfosClub,   Color32::from_rgba_unmultiplied(20, 55, 115, 220),  Color32::from_rgba_unmultiplied(35, 80, 160, 240)),
            ("👥", "Composition", "Gérer l'effectif",   Ecran::Composition, Color32::from_rgba_unmultiplied(15, 90, 40,  220),  Color32::from_rgba_unmultiplied(20, 125, 55,  240)),
            ("📅", "Calendrier",  "Prochains matchs",   Ecran::Calendrier,  Color32::from_rgba_unmultiplied(110, 65, 10, 220),  Color32::from_rgba_unmultiplied(150, 90, 15,  240)),
            ("💰", "Transferts",  "Recruter & vendre",  Ecran::Mercato,     Color32::from_rgba_unmultiplied(70, 20, 90,  220),  Color32::from_rgba_unmultiplied(100, 30, 130, 240)),
        ];
        for (i, (icon, titre, sous_titre, cible, bg, bg_hover)) in nav.iter().enumerate() {
            if carte_nav(&mut cols[i], icon, titre, sous_titre, *bg, *bg_hover) {
                action = Some(cible.clone());
            }
        }
    });

    if let Some(e) = action {
        *ecran_actuel = e;
    }
}

// ─────────────────────────────────────────────
//  Carte : Prochain Match + bouton JOUER
// ─────────────────────────────────────────────
fn render_prochain_match_card(
    ui: &mut Ui,
    club: &Club,
    next_game: &Option<Match>,
    classement: &Vec<Club>,
    card_height: f32,
    journee_actuelle: i32,
) -> bool {
    let mut play_clicked = false;

    egui::Frame::none()
        .fill(Color32::from_rgba_unmultiplied(18, 18, 38, 220))
        .rounding(14.0)
        .stroke(Stroke::new(1.5, Color32::from_rgba_unmultiplied(80, 80, 160, 180)))
        .inner_margin(egui::Margin::symmetric(20.0, 16.0))
        .show(ui, |ui| {
            ui.set_min_height(card_height - 32.0);

            // — Titre + journée + badge domicile/extérieur
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("⚽  PROCHAIN MATCH")
                        .font(FontId::proportional(15.0))
                        .color(Color32::LIGHT_BLUE)
                        .strong(),
                );
                if let Some(m) = next_game {
                    let est_domicile = club.id == Some(m.club_domicile_id);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (badge, badge_color) = if est_domicile {
                            ("🏠 Domicile", Color32::from_rgb(80, 200, 100))
                        } else {
                            ("✈ Extérieur", Color32::from_rgb(220, 160, 50))
                        };
                        ui.label(RichText::new(badge).font(FontId::proportional(12.0)).color(badge_color).strong());
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("Journée {}", m.journee)).font(FontId::proportional(12.0)).color(Color32::GRAY));
                    });
                }
            });
            ui.separator();
            ui.add_space(12.0);

            match next_game {
                Some(m) => {
                    // — Logos + noms
                    ui.horizontal(|ui| {
                        let logo_dom = format!("file:/{}", m.club_domicile_logo);
                        ui.add(egui::Image::new(&logo_dom).fit_to_exact_size(egui::vec2(52.0, 52.0)));
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(&m.club_domicile_nom)
                                .font(FontId::proportional(19.0))
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let logo_ext = format!("file:/{}", m.club_exterieur_logo);
                            ui.add(egui::Image::new(&logo_ext).fit_to_exact_size(egui::vec2(52.0, 52.0)));
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(&m.club_exterieur_nom)
                                    .font(FontId::proportional(19.0))
                                    .strong()
                                    .color(Color32::WHITE),
                            );
                        });
                    });

                    ui.add_space(4.0);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("—  VS  —").font(FontId::proportional(13.0)).color(Color32::GRAY));
                    });

                    if let Some(date) = &m.date_coup_envoi {
                        ui.add_space(4.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(format!("📅  {}", date)).font(FontId::proportional(13.0)).color(Color32::from_gray(155)));
                        });
                    }

                    // — Infos adversaire
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);

                    let est_domicile = club.id == Some(m.club_domicile_id);
                    let adversaire_id = if est_domicile { m.club_exterieur_id } else { m.club_domicile_id };
                    if let Some((pos, adv)) = classement.iter().enumerate()
                        .find(|(_, c)| c.id == Some(adversaire_id))
                        .map(|(i, c)| (i + 1, c))
                    {
                        let pos_color = if pos <= 3 { Color32::from_rgb(60, 210, 90) }
                                        else if pos <= 5 { Color32::from_rgb(60, 150, 230) }
                                        else if pos >= 16 { Color32::from_rgb(220, 70, 70) }
                                        else { Color32::GRAY };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("🎯 Adversaire :").font(FontId::proportional(12.5)).color(Color32::GRAY));
                            ui.add_space(4.0);
                            ui.label(RichText::new(format!("{}ème", pos)).font(FontId::proportional(12.5)).color(pos_color).strong());
                            ui.label(RichText::new(format!("• {} pts", adv.points)).font(FontId::proportional(12.5)).color(Color32::LIGHT_BLUE));
                        });
                    }

                    // — Progression de saison
                    ui.add_space(8.0);
                    let progress = (journee_actuelle as f32 - 1.0) / 34.0;
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("📊 Saison :").font(FontId::proportional(12.5)).color(Color32::GRAY));
                        ui.add_space(4.0);
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .desired_width(ui.available_width() - 60.0)
                                .fill(Color32::from_rgb(50, 120, 200))
                                .text(RichText::new(format!("J{} / 34", journee_actuelle - 1)).font(FontId::proportional(11.0))),
                        );
                    });
                }
                None => {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(RichText::new("Aucun match programmé").font(FontId::proportional(17.0)).color(Color32::GRAY));
                        ui.add_space(20.0);
                    });
                }
            }

            ui.add_space(14.0);

            // — Bouton JOUER
            ui.vertical_centered(|ui| {
                let btn = egui::Button::new(
                    RichText::new("▶   JOUER LE MATCH")
                        .font(FontId::proportional(18.0))
                        .strong()
                        .color(Color32::from_rgb(180, 255, 180)),
                )
                .fill(Color32::from_rgba_unmultiplied(15, 120, 45, 235))
                .stroke(Stroke::new(1.5, Color32::from_rgb(30, 200, 80)))
                .rounding(10.0)
                .min_size(Vec2::new(ui.available_width() * 0.72, 44.0));

                if ui.add(btn).clicked() {
                    play_clicked = true;
                }
            });
        });

    play_clicked
}

// ─────────────────────────────────────────────
//  Carte : Classement Ligue 1
// ─────────────────────────────────────────────
fn render_classement_card(ui: &mut Ui, classement: &Vec<Club>, journee_actuelle: i32, card_height: f32) -> bool {
    let mut voir_tout = false;

    egui::Frame::none()
        .fill(Color32::from_rgba_unmultiplied(18, 18, 38, 220))
        .rounding(14.0)
        .stroke(Stroke::new(1.5, Color32::from_rgba_unmultiplied(120, 100, 20, 180)))
        .inner_margin(egui::Margin::symmetric(14.0, 14.0))
        .show(ui, |ui| {
            ui.set_min_height(card_height - 28.0); // 28 = inner_margin haut+bas

            // — Titre
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("🏆  CLASSEMENT LIGUE 1")
                        .font(FontId::proportional(15.0))
                        .color(Color32::GOLD)
                        .strong(),
                );
            });
            ui.separator();
            ui.add_space(4.0);

            // — En-tête colonnes
            ui.horizontal(|ui| {
                ui.add_sized([26.0, 16.0], egui::Label::new(RichText::new("#").font(FontId::proportional(11.0)).color(Color32::GRAY).strong()));
                ui.add_sized([26.0, 16.0], egui::Label::new(RichText::new("").size(11.0)));
                ui.add_sized([110.0, 16.0], egui::Label::new(RichText::new("Club").font(FontId::proportional(11.0)).color(Color32::GRAY).strong()));
                ui.add_sized([26.0, 16.0], egui::Label::new(RichText::new("MJ").font(FontId::proportional(11.0)).color(Color32::GRAY).strong()));
                ui.add_sized([30.0, 16.0], egui::Label::new(RichText::new("DB").font(FontId::proportional(11.0)).color(Color32::GRAY).strong()));
                ui.add_sized([28.0, 16.0], egui::Label::new(RichText::new("Pts").font(FontId::proportional(11.0)).color(Color32::GRAY).strong()));
            });
            ui.separator();

            let mj = if journee_actuelle > 1 { journee_actuelle - 1 } else { 0 };

            // Hauteur scroll = hauteur totale allouée moins les widgets titre/séparateurs (~80px)
            let scroll_h = (card_height - 80.0).max(100.0);
            egui::ScrollArea::vertical()
                .max_height(scroll_h)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for (i, club) in classement.iter().enumerate() {
                        let pos = i + 1;

                        let row_bg = if i % 2 == 0 {
                            Color32::from_rgba_unmultiplied(255, 255, 255, 7)
                        } else {
                            Color32::TRANSPARENT
                        };

                        egui::Frame::none().fill(row_bg).rounding(4.0).show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.set_min_height(28.0);

                                // Position
                                let pos_color = if pos <= 3 {
                                    Color32::from_rgb(60, 210, 90)
                                } else if pos <= 5 {
                                    Color32::from_rgb(60, 150, 230)
                                } else if pos >= 16 {
                                    Color32::from_rgb(220, 70, 70)
                                } else {
                                    Color32::GRAY
                                };
                                ui.add_sized([26.0, 24.0], egui::Label::new(
                                    RichText::new(pos.to_string()).font(FontId::proportional(12.0)).color(pos_color).strong(),
                                ));

                                // Logo
                                let logo = format!("file:/{}", club.url_logo);
                                ui.add_sized([26.0, 24.0], egui::Image::new(&logo).fit_to_exact_size(egui::vec2(20.0, 20.0)));

                                // Nom
                                ui.add_sized([110.0, 24.0], egui::Label::new(
                                    RichText::new(&club.nom_court).font(FontId::proportional(12.5)).color(Color32::WHITE),
                                ));

                                // MJ
                                ui.add_sized([26.0, 24.0], egui::Label::new(
                                    RichText::new(mj.to_string()).font(FontId::proportional(11.5)).color(Color32::GRAY),
                                ));

                                // Diff buts
                                let db = club.buts_marques - club.buts_encaisses;
                                let db_color = if db > 0 { Color32::from_rgb(80, 200, 80) }
                                               else if db < 0 { Color32::from_rgb(210, 80, 80) }
                                               else { Color32::GRAY };
                                ui.add_sized([30.0, 24.0], egui::Label::new(
                                    RichText::new(format!("{:+}", db)).font(FontId::proportional(11.5)).color(db_color),
                                ));

                                // Points
                                ui.add_sized([28.0, 24.0], egui::Label::new(
                                    RichText::new(club.points.to_string())
                                        .font(FontId::proportional(13.0))
                                        .strong()
                                        .color(Color32::LIGHT_BLUE),
                                ));
                            });
                        });
                    }
                });
        });

    voir_tout
}

// ─────────────────────────────────────────────
//  Carte de navigation (bas de page)
// ─────────────────────────────────────────────
fn carte_nav(ui: &mut Ui, icon: &str, titre: &str, sous_titre: &str, bg: Color32, bg_hover: Color32) -> bool {
    let taille = Vec2::new(ui.available_width(), 112.0);
    let (id, rect) = ui.allocate_space(taille);
    let response = ui.interact(rect, id, egui::Sense::click());

    let bg_final = if response.hovered() { bg_hover } else { bg };
    let border = if response.hovered() {
        Color32::from_white_alpha(130)
    } else {
        Color32::from_white_alpha(50)
    };

    ui.painter().rect_filled(rect, 12.0, bg_final);
    ui.painter().rect_stroke(rect, 12.0, Stroke::new(1.5, border));

    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(12.0);
            ui.label(RichText::new(icon).font(FontId::proportional(30.0)));
            ui.add_space(4.0);
            ui.label(RichText::new(titre).font(FontId::proportional(16.0)).strong().color(Color32::WHITE));
            ui.add_space(2.0);
            ui.label(RichText::new(sous_titre).font(FontId::proportional(11.0)).color(Color32::from_white_alpha(170)));
        });
    });

    response.clicked()
}

// ─────────────────────────────────────────────
//  Header : logo + nom + budget + étoiles
// ─────────────────────────────────────────────
fn render_header(ui: &mut Ui, club: &Club, classement: &Vec<Club>, journee_actuelle: i32) {
    // Calcul position du club dans le classement
    let position = classement.iter().position(|c| c.id == club.id).map(|i| i + 1);
    let points = classement.iter().find(|c| c.id == club.id).map(|c| c.points).unwrap_or(club.points);

    egui::Frame::none()
        .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 160))
        .inner_margin(egui::Margin::symmetric(20.0, 0.0))
        .show(ui, |ui| {
            ui.set_min_height(90.0);
            ui.horizontal(|ui| {
                ui.add_space(0.0);

                // — Logo
                let logo_path = format!("file:/{}", club.url_logo);
                ui.add(
                    egui::Image::new(&logo_path)
                        .fit_to_exact_size(Vec2::new(70.0, 70.0))
                        .rounding(6.0),
                );
                ui.add_space(14.0);

                // — Nom du club
                ui.vertical(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        RichText::new(&club.nom)
                            .font(FontId::proportional(28.0))
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.label(
                        RichText::new("Ligue 1 • Saison 2025 / 2026")
                            .font(FontId::proportional(12.0))
                            .color(Color32::from_gray(150)),
                    );
                });

                // — Stats à droite (pills)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(6.0);

                    // Pill journée
                    stat_pill(ui,
                        "📅",
                        &format!("J.{} / 34", (journee_actuelle - 1).max(0)),
                        Color32::from_rgba_unmultiplied(30, 60, 120, 200),
                        Color32::from_rgb(120, 170, 255),
                    );
                    ui.add_space(8.0);

                    // Pill classement
                    let (pos_str, pos_color) = match position {
                        Some(1)            => ("1er".to_string(),   Color32::from_rgb(60, 210, 90)),
                        Some(p) if p <= 3  => (format!("{}e", p),   Color32::from_rgb(60, 210, 90)),
                        Some(p) if p <= 5  => (format!("{}e", p),   Color32::from_rgb(60, 150, 230)),
                        Some(p) if p >= 16 => (format!("{}e", p),   Color32::from_rgb(220, 70, 70)),
                        Some(p)            => (format!("{}e", p),   Color32::from_gray(210)),
                        None               => ("—".to_string(),     Color32::GRAY),
                    };
                    stat_pill(ui,
                        "🏆",
                        &format!("{} • {} pts", pos_str, points),
                        Color32::from_rgba_unmultiplied(80, 60, 10, 200),
                        pos_color,
                    );
                    ui.add_space(8.0);

                    // Pill budget
                    let budget_m = club.budget_eur / 1_000_000;
                    let budget_color = if budget_m >= 100 { Color32::from_rgb(80, 220, 100) }
                                       else if budget_m >= 30 { Color32::GOLD }
                                       else { Color32::from_rgb(220, 100, 80) };
                    stat_pill(ui,
                        "💰",
                        &format!("{} M€", budget_m),
                        Color32::from_rgba_unmultiplied(20, 60, 20, 200),
                        budget_color,
                    );
                });
            });
        });
}

/// Petite pill de stat avec icône + valeur colorée
fn stat_pill(ui: &mut Ui, icon: &str, valeur: &str, bg: Color32, valeur_color: Color32) {
    egui::Frame::none()
        .fill(bg)
        .rounding(8.0)
        .inner_margin(egui::Margin::symmetric(12.0, 6.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).font(FontId::proportional(13.0)));
                ui.add_space(4.0);
                ui.label(RichText::new(valeur).font(FontId::proportional(13.5)).strong().color(valeur_color));
            });
        });
}

// Conservé pour compatibilité si appelé ailleurs
pub fn afficher_classement(ui: &mut egui::Ui, classement: &Vec<Club>, journee_actuelle: i32) {
    render_classement_card(ui, classement, journee_actuelle, 432.0);
}
