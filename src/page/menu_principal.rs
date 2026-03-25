use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke};
use crate::models::{Club, Ecran};

struct CarteMenu {
    icon: &'static str,
    titre: &'static str,
    sous_titre: &'static str,
    cible: Ecran,
    couleur: Color32,
    couleur_hover: Color32,
}

pub fn render(ui: &mut Ui, club: &Club, ecran_actuel: &mut Ecran) {
    // Fond pelouse avec overlay sombre
    egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, ui.max_rect());

    ui.painter().rect_filled(
        ui.max_rect(),
        0.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 165),
    );

    render_header(ui, club);
    ui.add_space(28.0);

    let cartes: Vec<CarteMenu> = vec![
        CarteMenu {
            icon: "🏟",
            titre: "Infos Club",
            sous_titre: "Stade & finances",
            cible: Ecran::InfosClub,
            couleur: Color32::from_rgba_unmultiplied(20, 55, 115, 220),
            couleur_hover: Color32::from_rgba_unmultiplied(35, 80, 160, 240),
        },
        CarteMenu {
            icon: "👥",
            titre: "Composition",
            sous_titre: "Gérer l'effectif",
            cible: Ecran::Composition,
            couleur: Color32::from_rgba_unmultiplied(20, 55, 115, 220),
            couleur_hover: Color32::from_rgba_unmultiplied(35, 80, 160, 240),
        },
        CarteMenu {
            icon: "📅",
            titre: "Calendrier",
            sous_titre: "Prochains matchs",
            cible: Ecran::Calendrier,
            couleur: Color32::from_rgba_unmultiplied(20, 55, 115, 220),
            couleur_hover: Color32::from_rgba_unmultiplied(35, 80, 160, 240),
        },
        CarteMenu {
            icon: "🏆",
            titre: "Classement",
            sous_titre: "Table de la Ligue 1",
            cible: Ecran::Classement,
            couleur: Color32::from_rgba_unmultiplied(100, 70, 10, 220),
            couleur_hover: Color32::from_rgba_unmultiplied(140, 100, 15, 240),
        },
        CarteMenu {
            icon: "💰",
            titre: "Transferts",
            sous_titre: "Recruter & vendre",
            cible: Ecran::Mercato,
            couleur: Color32::from_rgba_unmultiplied(70, 20, 90, 220),
            couleur_hover: Color32::from_rgba_unmultiplied(100, 30, 130, 240),
        },
        CarteMenu {
            icon: "▶",
            titre: "PLAY",
            sous_titre: "Simuler le prochain match",
            cible: Ecran::ProchainMatch,
            couleur: Color32::from_rgba_unmultiplied(15, 120, 45, 230),
            couleur_hover: Color32::from_rgba_unmultiplied(20, 165, 60, 245),
        },
    ];

    for rangee in cartes.chunks(3) {
        ui.columns(3, |cols| {
            for (i, carte) in rangee.iter().enumerate() {
                if carte_menu(&mut cols[i], carte) {
                    *ecran_actuel = carte.cible.clone();
                }
            }
        });
        ui.add_space(14.0);
    }
}

fn carte_menu(ui: &mut Ui, carte: &CarteMenu) -> bool {
    let taille = Vec2::new(ui.available_width(), 128.0);
    let (id, rect) = ui.allocate_space(taille);
    let response = ui.interact(rect, id, egui::Sense::click());

    let bg = if response.hovered() { carte.couleur_hover } else { carte.couleur };
    let border = if response.hovered() {
        Color32::from_white_alpha(120)
    } else {
        Color32::from_white_alpha(55)
    };

    ui.painter().rect_filled(rect, 14.0, bg);
    ui.painter().rect_stroke(rect, 14.0, Stroke::new(1.5, border));

    // Contenu centré via child UI
    ui.allocate_ui_at_rect(rect, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(14.0);
            ui.label(RichText::new(carte.icon).font(FontId::proportional(34.0)));
            ui.add_space(4.0);
            let titre_color = if carte.titre == "PLAY" {
                Color32::from_rgb(180, 255, 180)
            } else {
                Color32::WHITE
            };
            ui.label(
                RichText::new(carte.titre)
                    .font(FontId::proportional(18.0))
                    .strong()
                    .color(titre_color),
            );
            ui.label(
                RichText::new(carte.sous_titre)
                    .font(FontId::proportional(11.5))
                    .color(Color32::from_white_alpha(185)),
            );
        });
    });

    response.clicked()
}

fn render_header(ui: &mut Ui, club: &Club) {
    // Bandeau semi-transparent
    let header_rect = egui::Rect::from_min_size(
        ui.cursor().min,
        Vec2::new(ui.available_width(), 95.0),
    );
    ui.painter().rect_filled(
        header_rect,
        0.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 130),
    );

    ui.horizontal(|ui| {
        ui.add_space(18.0);

        let logo_path = format!("file://.{}", club.url_logo);
        ui.add(
            egui::Image::new(&logo_path)
                .fit_to_exact_size(Vec2::new(75.0, 75.0))
                .rounding(6.0),
        );

        ui.add_space(18.0);

        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new(&club.nom)
                    .font(FontId::proportional(32.0))
                    .strong()
                    .color(Color32::WHITE),
            );
            ui.label(
                RichText::new(format!("Budget : {} M€", club.budget_eur / 1_000_000))
                    .font(FontId::proportional(17.0))
                    .color(Color32::GOLD),
            );
        });

        // Réputation à droite
        let nb_etoiles: usize = match club.reputation {
            90..=100 => 5,
            70..=89 => 4,
            60..=69 => 3,
            40..=59 => 2,
            20..=39 => 1,
            _ => 0,
        };
        let etoiles = format!("{}{}", "★".repeat(nb_etoiles), "☆".repeat(5 - nb_etoiles));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(18.0);
            ui.label(
                RichText::new(etoiles)
                    .font(FontId::proportional(24.0))
                    .color(Color32::GOLD),
            );
        });
    });
}
