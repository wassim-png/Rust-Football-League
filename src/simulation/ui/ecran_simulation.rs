use eframe::egui;
use egui::{Ui, RichText, Color32, FontId, Frame, Stroke, Vec2, Align2};
use crate::models::{Club, Ecran, ResultatMatchJournee};
pub fn render(
    ui: &mut egui::Ui, 
    resultats: &Vec<ResultatMatchJournee>, 
    journee_actuelle: i32, 
    total_journees: i32
) -> bool {
    let mut passer_suivante = false;

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading(
            RichText::new("📊 RÉSULTATS DE LA JOURNÉE")
                .font(FontId::proportional(26.0))
                .strong()
                .color(Color32::WHITE),
        );
        // On affiche journee_actuelle - 1 car on vient de la simuler
        ui.label(
            RichText::new(format!("Journée {} sur {}", journee_actuelle - 1, total_journees))
                .font(FontId::proportional(14.0))
                .color(Color32::GRAY),
        );
        ui.add_space(20.0);
    });

   egui::ScrollArea::vertical()
    .max_height(450.0)
    .auto_shrink([false; 2])
    .show(ui, |ui| {
        for resultat in resultats {
            let bg_color = if resultat.est_match_utilisateur {
                Color32::from_rgba_unmultiplied(35, 75, 35, 180) 
            } else {
                Color32::from_rgba_unmultiplied(45, 45, 45, 200) 
            };

            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.style_mut().visuals.widgets.noninteractive.bg_fill = bg_color;

                ui.horizontal(|ui| {
                    let largeur_disponible = ui.available_width();
                    
                    // --- 1. DOMICILE (42%) ---
                    ui.allocate_ui(Vec2::new(largeur_disponible * 0.42, 35.0), |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(&resultat.nom_domicile).font(FontId::proportional(16.0)).strong());
                            
                            // LOGO DOMICILE AVEC LE FORMAT file://.
                            if let Some(url) = &resultat.url_logo_domicile {
                                let chemin_complet = format!("file://.{}", url);
                                ui.add(egui::Image::new(chemin_complet).max_size(Vec2::new(24.0, 24.0)));
                            } else {
                                ui.label("⚽"); 
                            }
                        });
                    });

                    // --- 2. SCORE CENTRAL (16%) ---
                    ui.allocate_ui(Vec2::new(largeur_disponible * 0.16, 35.0), |ui| {
                        ui.centered_and_justified(|ui| {
                            let score = format!("{} - {}", resultat.buts_domicile, resultat.buts_exterieur);
                            ui.label(
                                RichText::new(score)
                                    .font(FontId::proportional(20.0))
                                    .color(Color32::YELLOW)
                                    .background_color(Color32::BLACK)
                                    .strong(),
                            );
                        });
                    });

                    // --- 3. EXTÉRIEUR (Reste) ---
                    ui.allocate_ui(Vec2::new(ui.available_width(), 35.0), |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            // LOGO EXTÉRIEUR AVEC LE FORMAT file://.
                            if let Some(url) = &resultat.url_logo_exterieur {
                                let chemin_complet = format!("file://.{}", url);
                                ui.add(egui::Image::new(chemin_complet).max_size(Vec2::new(24.0, 24.0)));
                            } else {
                                ui.label("⚽");
                            }
                            ui.label(RichText::new(&resultat.nom_exterieur).font(FontId::proportional(16.0)).strong());
                        });
                    });
                });
            });
            ui.add_space(6.0);
        }
    });

    ui.add_space(30.0);

    // --- 3. BOUTON DE NAVIGATION ---
    ui.vertical_centered(|ui| {
        let texte_bouton = if journee_actuelle <= total_journees {
            "➡ PROCHAINE JOURNÉE"
        } else {
            "🏆 VOIR LE CLASSEMENT FINAL"
        };

        let btn = egui::Button::new(
            RichText::new(texte_bouton)
                .font(FontId::proportional(18.0))
                .strong()
                .color(Color32::WHITE)
        )
        .fill(Color32::from_rgb(30, 100, 200))
        .min_size(Vec2::new(280.0, 45.0))
        .rounding(8.0);

        if ui.add(btn).clicked() {
            passer_suivante = true;
        }
    });

    passer_suivante
}