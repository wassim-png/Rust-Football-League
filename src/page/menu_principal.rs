use eframe::egui;
use egui::{Ui, Color32, RichText, FontId, Vec2, Stroke};
use crate::models::{Club, Ecran, Match};

struct CarteMenu {
    icon: &'static str,
    titre: &'static str,
    sous_titre: &'static str,
    cible: Ecran,
    couleur: Color32,
    couleur_hover: Color32,
}

pub fn render(ui: &mut Ui, club: &Club, ecran_actuel: &mut Ecran, next_game : &Option<Match>, classement :&Vec<Club>, journee_actuelle: i32) {
    let rect_ecran = ui.max_rect();
    
   egui::Image::new("file://assets/pelouse.jpg")
        .maintain_aspect_ratio(false)
        .max_size(ui.available_size())
        .paint_at(ui, ui.max_rect());

   ui.painter().rect_filled(
        rect_ecran,
        0.0,
        Color32::from_rgba_unmultiplied(50, 50, 0, 165),
    );

    // 2. PLACEMENT ABSOLU DU BANDEAU (TOUT EN HAUT)
    // On crée une boîte de 95px de haut qui commence tout en haut de l'écran (rect_ecran.min)
    let header_rect = egui::Rect::from_min_size(
        rect_ecran.min,
        Vec2::new(rect_ecran.width(), 95.0),
    );
ui.allocate_ui_at_rect(header_rect, |ui| {
        render_header(ui, club);
    });

    egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 30, 220)) // Fond gris très foncé, presque opaque
            .rounding(15.0) // Bords bien arrondis
            .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY)) // Petite bordure grise
            .inner_margin(egui::Margin::symmetric(50.0, 20.0)) // Marge interne (Largeur, Hauteur)
            .show(ui, |ui| {
                
                match next_game {
                Some(m) => {
                    // 1. LE TITRE AVEC LE NUMÉRO DE LA JOURNÉE
                    ui.label(
                        egui::RichText::new(format!("⚽ PROCHAIN MATCH - Journée {}", m.journee))
                            .font(egui::FontId::proportional(20.0))
                            .color(egui::Color32::LIGHT_BLUE)
                    );
                    
                    ui.add_space(20.0);
                    
                    // 2. L'AFFICHE (Logos et Noms sur la même ligne)
                    ui.horizontal(|ui| {
                        
                        // --- ÉQUIPE DOMICILE ---
                        let chemin_logo_dom = format!("file:/{}", m.club_domicile_logo);
                        ui.add(egui::Image::new(&chemin_logo_dom).fit_to_exact_size(egui::vec2(50.0, 50.0)));
                        
                        ui.label(
                            egui::RichText::new(&m.club_domicile_nom)
                                .font(egui::FontId::proportional(30.0))
                                .strong()
                                .color(egui::Color32::WHITE)
                        );
                        
                        // --- LE "VS" AU MILIEU ---
                        ui.label(
                            egui::RichText::new("  VS  ")
                                .font(egui::FontId::proportional(20.0))
                                .color(egui::Color32::GRAY)
                        );
                        
                        // --- ÉQUIPE EXTÉRIEURE ---
                        ui.label(
                            egui::RichText::new(&m.club_exterieur_nom)
                                .font(egui::FontId::proportional(30.0))
                                .strong()
                                .color(egui::Color32::WHITE)
                        );
                        
                        let chemin_logo_ext = format!("file:/{}", m.club_exterieur_logo);
                        ui.add(egui::Image::new(&chemin_logo_ext).fit_to_exact_size(egui::vec2(50.0, 50.0)));
                    });

                    ui.add_space(15.0);

                    // 3. LA DATE (Comme c'est un Option<String>, on fait un "if let")
                    if let Some(date) = &m.date_coup_envoi {
                        ui.label(
                            egui::RichText::new(format!("📅 Coup d'envoi : {}", date))
                                .font(egui::FontId::proportional(18.0))
                                .color(egui::Color32::GRAY)
                        );
                    }
                }
                None => {
                    // S'il n'y a pas de match programmé (ex: fin de saison)
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Aucun match au calendrier")
                            .font(egui::FontId::proportional(25.0))
                            .color(egui::Color32::GRAY)
                    );
                    ui.add_space(20.0);
                }
            }
        });

        afficher_classement(ui, classement, journee_actuelle);
            

    ui.add_space(20.0);

    let cartes: Vec<CarteMenu> = vec![
        CarteMenu {
            icon: "🏢",
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
    // On utilise egui::Frame pour gérer le fond noir semi-transparent automatiquement
    egui::Frame::none()
        .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 130))
        .inner_margin(egui::Margin::symmetric(20.0, 15.0)) // Marges internes
        .show(ui, |ui| {
            
            // On fixe la hauteur minimale de ce bandeau
            ui.set_min_height(90.0);

            ui.horizontal(|ui| {
                // Attention ici à ton chemin d'image. J'ai enlevé le "." qui traînait
                let logo_path = format!("file:/{}", club.url_logo); 
                
                ui.add(
                    egui::Image::new(&logo_path)
                        .fit_to_exact_size(Vec2::new(75.0, 75.0))
                        .rounding(6.0),
                );

                ui.add_space(20.0);

                ui.vertical(|ui| {
                    ui.add_space(10.0); // Pour centrer le texte verticalement par rapport au logo
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

                // --- Étoiles de réputation à droite ---
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
                    ui.label(
                        RichText::new(etoiles)
                            .font(FontId::proportional(24.0))
                            .color(Color32::GOLD),
                    );
                });
            });
        });
    }



    pub fn afficher_classement(ui: &mut egui::Ui, classement: &Vec<Club>, journee_actuelle: i32) {
    
    // 🌟 On centre tout le bloc et on bloque la largeur à 450 pixels maximum
    ui.vertical_centered(|ui| {
        ui.set_max_width(450.0);

        // --- 1. LE RECTANGLE CONTENEUR ---
        egui::Frame::none()
            .fill(egui::Color32::from_gray(30))
            .rounding(10.0)
            .inner_margin(10.0)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_gray(60)))
            .show(ui, |ui| {
                
                ui.heading("Classement Ligue 1");
                ui.add_space(10.0);

                // --- 2. L'EN-TÊTE ---
                ui.horizontal(|ui| {
                    ui.set_min_height(25.0);
                    ui.add_sized([30.0, 20.0], egui::Label::new(egui::RichText::new("Pos").strong()));
                    ui.add_sized([40.0, 20.0], egui::Label::new(egui::RichText::new("Logo").strong()));
                    ui.add_sized([150.0, 20.0], egui::Label::new(egui::RichText::new("Club").strong()));
                    ui.add_sized([35.0, 20.0], egui::Label::new(egui::RichText::new("MJ").strong()));
                    ui.add_sized([35.0, 20.0], egui::Label::new(egui::RichText::new("DB").strong()));
                    ui.add_sized([40.0, 20.0], egui::Label::new(egui::RichText::new("Pts").strong()));
                });
                
                ui.separator();

                // --- 3. LES DONNÉES ---
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        
                        for (index, ligne) in classement.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.set_min_height(35.0);

                                // A. Position (Calculée via l'index : 0 devient 1er)
                                let position = index + 1;
                                let pos_text = egui::RichText::new(position.to_string());
                                let pos_text = if position <= 3 { 
                                    pos_text.color(egui::Color32::GREEN) // Podium en vert
                                } else if position >= 16 { 
                                    pos_text.color(egui::Color32::RED) // Relégation en rouge
                                } else { 
                                    pos_text 
                                };
                                ui.add_sized([30.0, 30.0], egui::Label::new(pos_text));

                                // B. Logo
                                let chemin_logo = format!("file:/{}", ligne.url_logo);
                                ui.add_sized([40.0, 30.0], egui::Image::new(&chemin_logo).fit_to_exact_size(egui::vec2(25.0, 25.0)));

                                // C. Club
                                ui.add_sized([150.0, 30.0], egui::Label::new(egui::RichText::new(&ligne.nom_court).strong()));

                                // D. Matchs Joués (Journée actuelle moins 1)
                                let mj = if journee_actuelle > 1 { journee_actuelle - 1 } else { 0 };
                                ui.add_sized([35.0, 30.0], egui::Label::new(mj.to_string()));

                                // E. Différence de buts
                                let db = ligne.buts_marques - ligne.buts_encaisses;
                                let db_text = egui::RichText::new(format!("{:+} ", db));
                                
                                let db_text = if db > 0 { 
                                    db_text.color(egui::Color32::from_rgb(0, 180, 0)) 
                                } else if db < 0 { 
                                    db_text.color(egui::Color32::from_rgb(220, 0, 0)) 
                                } else { 
                                    db_text 
                                };
                                
                                ui.add_sized([35.0, 30.0], egui::Label::new(db_text));

                                // F. Points
                                ui.add_sized([40.0, 30.0], egui::Label::new(
                                    egui::RichText::new(ligne.points.to_string())
                                        .font(egui::FontId::proportional(18.0))
                                        .strong()
                                        .color(egui::Color32::LIGHT_BLUE)
                                ));
                            });
                            
                            // Ligne de séparation sauf pour le dernier
                            if index < classement.len() - 1 {
                                 ui.separator();
                            }
                        }
                    });
            });
    });
}

        


