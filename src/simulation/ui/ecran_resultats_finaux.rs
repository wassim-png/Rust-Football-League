use eframe::egui;
use eframe::egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2};
use crate::models::{Club};

const OR: Color32 = Color32::from_rgb(212, 175, 55);

pub fn render(ui: &mut Ui, liste_equipes: &[Club], equipe_choisie: &Option<Club>) -> bool {
    let mut restart_game = false;
    let mut classement = liste_equipes.to_vec();
   
    classement.sort_by(|a, b| {
        let diff_a = a.buts_marques - a.buts_encaisses;
        let diff_b = b.buts_marques - b.buts_encaisses;
        b.points
            .cmp(&a.points)
            .then(diff_b.cmp(&diff_a))
            .then(b.buts_marques.cmp(&a.buts_marques))
    });

    let champion = classement.first();
    let pire_defense = classement.iter().max_by_key(|c| c.buts_encaisses);
    let meilleure_attaque = classement.iter().max_by_key(|c| c.buts_marques);
    let club_utilisateur = equipe_choisie.as_ref();
    let rang_utilisateur = club_utilisateur.and_then(|u| classement.iter().position(|c| c.id == u.id));

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        ui.label(
            RichText::new("FIN DE LA SAISON")
                .font(FontId::proportional(32.0))
                .strong()
                .color(OR),
        );
        ui.add_space(10.0);
        ui.label(
            RichText::new("Classement complet et statistiques finales")
                .font(FontId::proportional(16.0))
                .color(Color32::LIGHT_GRAY),
        );

        ui.add_space(30.0);

        ui.horizontal(|ui| {
            
            ui.vertical(|ui| {
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(15, 15, 25, 240))
                    .stroke(Stroke::new(1.0, Color32::from_gray(60)))
                    .rounding(10.0)
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Classement Final")
                                .font(FontId::proportional(20.0))
                                .strong()
                                .color(Color32::WHITE),
                        );
                        ui.add_space(10.0);

                        egui::ScrollArea::vertical()
                           
                            .show(ui, |ui| {
                                egui::Grid::new("grid_classement_final")
                                    .num_columns(6)
                                    .spacing([15.0, 8.0])
                                    .show(ui, |ui| {
                                        ui.label(RichText::new("#").strong().color(Color32::GRAY));
                                        ui.label(RichText::new("Club").strong().color(Color32::GRAY));
                                        ui.label(RichText::new("PTS").strong().color(OR));
                                        ui.label(RichText::new("BP").strong().color(Color32::GRAY));
                                        ui.label(RichText::new("BC").strong().color(Color32::GRAY));
                                        ui.label(RichText::new("DIFF").strong().color(Color32::GRAY));
                                        ui.end_row();

                                        for (i, club) in classement.iter().enumerate() {
                                            let rang = i + 1;
                                            let diff = club.buts_marques - club.buts_encaisses;

                                            let couleur_txt = match rang {
                                                1 => OR,
                                                2..=4 => Color32::from_rgb(100, 150, 255), // Europe
                                                16..=18 => Color32::from_rgb(200, 50, 50), // Relégation
                                                _ => Color32::WHITE,
                                            };

                                            let mut texte_rang = rang.to_string();
                                            if rang == 1 {
                                                texte_rang = "1er".to_string();
                                            } else if rang == 2 {
                                                texte_rang = "2ème".to_string();
                                            } else if rang == 3 {
                                                texte_rang = "3ème".to_string();
                                            }

                                            ui.label(RichText::new(texte_rang).color(couleur_txt));
                                            
                                           
                                            if let Some(user_club) = club_utilisateur {
                                                if club.id == user_club.id {
                                                    ui.label(RichText::new(&club.nom).strong().color(Color32::from_rgb(100, 255, 100)));
                                                } else {
                                                    ui.label(RichText::new(&club.nom).color(couleur_txt));
                                                }
                                            } else {
                                                ui.label(RichText::new(&club.nom).color(couleur_txt));
                                            }

                                            ui.label(RichText::new(club.points.to_string()).strong().color(OR));
                                            ui.label(RichText::new(club.buts_marques.to_string()).color(Color32::LIGHT_GRAY));
                                            ui.label(RichText::new(club.buts_encaisses.to_string()).color(Color32::LIGHT_GRAY));
                                            
                                            let cb = if diff > 0 { Color32::from_rgb(76, 175, 80) } else if diff < 0 { Color32::from_rgb(244, 67, 54) } else { Color32::LIGHT_GRAY };
                                            ui.label(RichText::new(if diff > 0 { format!("+{}", diff) } else { diff.to_string() }).color(cb));
                                            
                                            ui.end_row();
                                        }
                                    });
                            });
                    });
            });

            ui.add_space(30.0);

            ui.vertical(|ui| {
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(15, 15, 25, 240))
                    .stroke(Stroke::new(1.0, OR))
                    .rounding(10.0)
                    .inner_margin(15.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Tableau d'honneur")
                                .font(FontId::proportional(20.0))
                                .strong()
                                .color(OR),
                        );
                        ui.add_space(15.0);

                        if let Some(champ) = champion {
                            ui.label(RichText::new("[1] Champion").color(Color32::GRAY));
                            ui.label(RichText::new(&champ.nom).font(FontId::proportional(18.0)).strong().color(Color32::WHITE));
                            ui.label(RichText::new(format!("{} points", champ.points)).color(OR));
                            ui.add_space(10.0);
                        }

                        if let Some(att) = meilleure_attaque {
                            ui.label(RichText::new("[+] Meilleure attaque").color(Color32::GRAY));
                            ui.label(RichText::new(&att.nom).font(FontId::proportional(16.0)).strong().color(Color32::WHITE));
                            ui.label(RichText::new(format!("{} buts marqués", att.buts_marques)).color(Color32::LIGHT_GRAY));
                            ui.add_space(10.0);
                        }

                        if let Some(def) = pire_defense {
                            ui.label(RichText::new("[-] Pire défense").color(Color32::GRAY));
                            ui.label(RichText::new(&def.nom).font(FontId::proportional(16.0)).strong().color(Color32::WHITE));
                            ui.label(RichText::new(format!("{} buts encaissés", def.buts_encaisses)).color(Color32::from_rgb(200, 80, 80)));
                            ui.add_space(10.0);
                        }

                        if let (Some(_u), Some(r)) = (club_utilisateur, rang_utilisateur) {
                            let actual_club = &classement[r];
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            ui.label(RichText::new("Votre bilan").color(Color32::GRAY));
                            ui.label(RichText::new(&actual_club.nom).font(FontId::proportional(18.0)).strong().color(Color32::from_rgb(100, 255, 100)));
                            ui.label(RichText::new(format!("Positions : {}ème", r + 1)).color(Color32::WHITE));
                            ui.label(RichText::new(format!("Points : {}", actual_club.points)).color(OR));
                        }
                    });

                ui.add_space(40.0);

                let btn_retour = egui::Button::new(
                    RichText::new("⬅ Retourner au Menu (Nouvelle Partie)")
                        .font(FontId::proportional(16.0))
                        .color(Color32::WHITE),
                )
                .fill(Color32::from_rgb(180, 40, 40))
                .min_size(Vec2::new(280.0, 45.0))
                .rounding(8.0);

                if ui.add(btn_retour).clicked() {
                    restart_game = true;
                }
            });
        });
    });

    restart_game
}
