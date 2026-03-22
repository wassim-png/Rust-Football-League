use eframe::egui::{self, Color32, Context, FontId, Frame, RichText, ScrollArea, Stroke, Ui};
use crate::models::{Club, Ecran, EtatMercato, Joueur, OngletMercato};

const OR: Color32 = Color32::from_rgb(212, 175, 55);
const FOND_SOMBRE: Color32 = Color32::from_rgb(18, 18, 28);
const FOND_CARTE: Color32 = Color32::from_rgb(28, 28, 42);
const VERT: Color32 = Color32::from_rgb(50, 180, 80);
const ROUGE_VIF: Color32 = Color32::from_rgb(200, 50, 50);
const BLEU: Color32 = Color32::from_rgb(40, 100, 200);

fn etoiles(rep: i32) -> String {
    let n = ((rep as f32 / 100.0) * 5.0).round() as usize;
    format!("{}{}", "★".repeat(n.min(5)), "☆".repeat(5 - n.min(5)))
}

fn fmt_eur(v: i64) -> String {
    if v >= 1_000_000 {
        format!("{:.1}M€", v as f64 / 1_000_000.0)
    } else if v >= 1_000 {
        format!("{}k€", v / 1_000)
    } else {
        format!("{}€", v)
    }
}

pub fn render(ctx: &Context, ui: &mut Ui, equipe: &mut Club, etat: &mut EtatMercato, ecran_actuel: &mut Ecran) {
    let rect = ui.max_rect();
    ui.painter().rect_filled(rect, 0.0, FOND_SOMBRE);
    ui.add_space(6.0);

    // --- Bouton retour en haut ---
    if ui.button(RichText::new("⬅  Retour").color(Color32::LIGHT_GRAY)).clicked() {
        *ecran_actuel = Ecran::MenuPrincipal;
    }
    ui.add_space(4.0);

    // --- Header ---
    Frame::none()
        .fill(Color32::from_rgb(10, 12, 22))
        .stroke(Stroke::new(1.5, OR))
        .inner_margin(12.0)
        .rounding(8.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("MARCHÉ DES TRANSFERTS")
                        .font(FontId::proportional(22.0))
                        .color(OR)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("Budget : {}", fmt_eur(equipe.budget_eur)))
                            .font(FontId::proportional(17.0))
                            .color(VERT),
                    );
                });
            });
            ui.label(
                RichText::new("Mercato d'Été  ·  1 Juillet → 31 Août")
                    .font(FontId::proportional(12.0))
                    .color(Color32::GRAY),
            );
        });

    ui.add_space(10.0);

    // --- Onglets ---
    ui.horizontal(|ui| {
        tab_btn(ui, "  Joueurs disponibles  ", etat.onglet == OngletMercato::JoueursDisponibles, || {
            etat.onglet = OngletMercato::JoueursDisponibles;
            etat.message = None;
        });
        let label_offres = if etat.offres_recues.is_empty() {
            "  Offres Reçues  ".to_string()
        } else {
            format!("  Offres Reçues ({})  ", etat.offres_recues.len())
        };
        tab_btn(ui, &label_offres, etat.onglet == OngletMercato::OffresRecues, || {
            etat.onglet = OngletMercato::OffresRecues;
            etat.message = None;
        });
    });

    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(1.5, OR),
    );
    ui.add_space(8.0);

    // --- Message feedback ---
    if let Some(msg) = &etat.message.clone() {
        let color = if msg.contains('✓') { VERT } else { ROUGE_VIF };
        Frame::none()
            .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 120))
            .rounding(6.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                ui.label(RichText::new(msg).color(color).font(FontId::proportional(14.0)));
            });
        ui.add_space(6.0);
    }

    // --- Contenu ---
    match etat.onglet {
        OngletMercato::JoueursDisponibles => render_joueurs(ui, etat),
        OngletMercato::OffresRecues => render_offres_recues(ui, equipe, etat),
    }

    // --- Modales ---
    render_modal(ctx, equipe, etat);
}

fn tab_btn(ui: &mut Ui, label: &str, actif: bool, mut on_click: impl FnMut()) {
    let (fill, text_color) = if actif {
        (OR, Color32::BLACK)
    } else {
        (Color32::from_rgb(35, 35, 55), Color32::LIGHT_GRAY)
    };
    let btn = egui::Button::new(
        RichText::new(label).color(text_color).font(FontId::proportional(13.0)),
    )
    .fill(fill)
    .stroke(Stroke::NONE)
    .rounding(egui::Rounding { nw: 6.0, ne: 6.0, sw: 0.0, se: 0.0 });
    if ui.add(btn).clicked() {
        on_click();
    }
}

fn badge_poste(ui: &mut Ui, poste: &str) {
    let color = match poste {
        "ATTAQUE" => Color32::from_rgb(190, 60, 60),
        "MILIEU" => Color32::from_rgb(60, 120, 200),
        "DEFENSE" => Color32::from_rgb(60, 160, 60),
        "GARDIEN" => Color32::from_rgb(170, 130, 30),
        _ => Color32::GRAY,
    };
    Frame::none()
        .fill(color)
        .rounding(4.0)
        .inner_margin(egui::Margin { left: 5.0, right: 5.0, top: 1.0, bottom: 1.0 })
        .show(ui, |ui| {
            ui.label(
                RichText::new(poste).font(FontId::proportional(11.0)).color(Color32::WHITE).strong(),
            );
        });
}

// ──────────────────────────────────────────────────────────
// Onglet : liste unifiée avec barre de recherche
// ──────────────────────────────────────────────────────────

fn render_joueurs(ui: &mut Ui, etat: &mut EtatMercato) {
    // Barre de recherche + filtre poste sur la même ligne
    ui.horizontal(|ui| {
        ui.label(RichText::new("🔍").font(FontId::proportional(15.0)));
        ui.add(
            egui::TextEdit::singleline(&mut etat.recherche)
                .hint_text("Rechercher un joueur...")
                .desired_width(200.0),
        );
        if !etat.recherche.is_empty() && ui.small_button("✕").clicked() {
            etat.recherche.clear();
        }

        ui.add_space(16.0);

        // Filtres de poste
        for (label, poste) in [
            ("Tous", None),
            ("GK", Some("GARDIEN")),
            ("DEF", Some("DEFENSE")),
            ("MIL", Some("MILIEU")),
            ("ATT", Some("ATTAQUE")),
        ] {
            let actif = etat.filtre_poste.as_deref() == poste;
            let (fill, text_color) = if actif {
                (OR, Color32::BLACK)
            } else {
                (Color32::from_rgb(40, 40, 60), Color32::LIGHT_GRAY)
            };
            let btn = egui::Button::new(
                RichText::new(label).color(text_color).font(FontId::proportional(12.0)),
            )
            .fill(fill)
            .stroke(Stroke::NONE)
            .rounding(4.0);
            if ui.add(btn).clicked() {
                etat.filtre_poste = poste.map(|s| s.to_string());
            }
        }
    });
    ui.add_space(6.0);

    let filtre_nom = etat.recherche.to_lowercase();
    let filtre_poste = etat.filtre_poste.clone();
    let indices_filtres: Vec<usize> = etat
        .tous_joueurs
        .iter()
        .enumerate()
        .filter(|(_, j)| {
            (filtre_nom.is_empty() || j.nom.to_lowercase().contains(&filtre_nom))
                && (filtre_poste.is_none() || filtre_poste.as_deref() == Some(j.poste.as_str()))
        })
        .map(|(i, _)| i)
        .collect();

    if indices_filtres.is_empty() {
        ui.label(RichText::new("Aucun joueur trouvé.").color(Color32::GRAY));
        return;
    }

    ScrollArea::vertical().id_source("scroll_joueurs").show(ui, |ui| {
        for idx in indices_filtres {
            let j = etat.tous_joueurs[idx].clone();
            carte_joueur(ui, idx, &j, etat);
            ui.add_space(4.0);
        }
    });
}

fn carte_joueur(ui: &mut Ui, idx: usize, j: &Joueur, etat: &mut EtatMercato) {
    let est_libre = j.club_nom.is_none();
    let stroke_color = if est_libre {
        Color32::from_rgb(50, 120, 50)
    } else {
        Color32::from_rgb(55, 55, 75)
    };

    Frame::none()
        .fill(FOND_CARTE)
        .stroke(Stroke::new(1.0, stroke_color))
        .rounding(8.0)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Nom + club ou badge "Libre"
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&j.nom)
                                .font(FontId::proportional(15.0))
                                .color(Color32::WHITE)
                                .strong(),
                        );
                        match &j.club_nom {
                            None => {
                                Frame::none()
                                    .fill(Color32::from_rgb(40, 110, 40))
                                    .rounding(4.0)
                                    .inner_margin(egui::Margin { left: 5.0, right: 5.0, top: 1.0, bottom: 1.0 })
                                    .show(ui, |ui| {
                                        ui.label(
                                            RichText::new("LIBRE")
                                                .font(FontId::proportional(10.0))
                                                .color(Color32::WHITE)
                                                .strong(),
                                        );
                                    });
                            }
                            Some(club) => {
                                ui.label(
                                    RichText::new(format!("— {}", club))
                                        .color(Color32::from_rgb(140, 140, 200))
                                        .font(FontId::proportional(13.0)),
                                );
                            }
                        }
                    });
                    // Infos : âge, poste, étoiles
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} ans", j.age))
                                .color(Color32::GRAY)
                                .font(FontId::proportional(12.0)),
                        );
                        ui.label(RichText::new("·").color(Color32::from_gray(60)));
                        badge_poste(ui, &j.poste);
                        ui.label(RichText::new("·").color(Color32::from_gray(60)));
                        ui.label(
                            RichText::new(etoiles(j.reputation))
                                .color(OR)
                                .font(FontId::proportional(12.0)),
                        );
                    });
                    // Valeur / Salaire
                    if est_libre {
                        ui.label(
                            RichText::new(format!(
                                "Prix : {}  ·  Salaire : {}/sem",
                                fmt_eur(j.valeur_marche_eur),
                                fmt_eur(j.salaire_semaine_eur)
                            ))
                            .color(Color32::from_rgb(170, 170, 170))
                            .font(FontId::proportional(12.0)),
                        );
                    } else {
                        ui.label(
                            RichText::new(format!(
                                "Valeur : {}  ·  Salaire : {}/sem",
                                fmt_eur(j.valeur_marche_eur),
                                fmt_eur(j.salaire_semaine_eur)
                            ))
                            .color(Color32::from_rgb(170, 170, 170))
                            .font(FontId::proportional(12.0)),
                        );
                    }
                });

                // Bouton d'action
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if est_libre {
                        let btn = egui::Button::new(
                            RichText::new("Recruter").color(Color32::WHITE).font(FontId::proportional(13.0)),
                        )
                        .fill(VERT)
                        .stroke(Stroke::NONE)
                        .rounding(6.0);
                        if ui.add(btn).clicked() {
                            etat.joueur_selectionne = Some(idx);
                            etat.message = None;
                        }
                    } else {
                        let btn = egui::Button::new(
                            RichText::new("Faire une offre")
                                .color(Color32::WHITE)
                                .font(FontId::proportional(13.0)),
                        )
                        .fill(BLEU)
                        .stroke(Stroke::NONE)
                        .rounding(6.0);
                        if ui.add(btn).clicked() {
                            etat.joueur_selectionne = Some(idx);
                            etat.offre_montant = j.valeur_marche_eur as f64;
                            etat.message = None;
                        }
                    }
                });
            });
        });
}

// ──────────────────────────────────────────────────────────
// Onglet : offres reçues
// ──────────────────────────────────────────────────────────

fn render_offres_recues(ui: &mut Ui, equipe: &mut Club, etat: &mut EtatMercato) {
    if etat.offres_recues.is_empty() {
        ui.add_space(30.0);
        ui.label(
            RichText::new("Aucune offre reçue pour le moment.")
                .color(Color32::GRAY)
                .font(FontId::proportional(15.0)),
        );
        return;
    }

    let mut action: Option<(usize, bool)> = None;

    ScrollArea::vertical().id_source("scroll_offres").show(ui, |ui| {
        for (i, offre) in etat.offres_recues.iter().enumerate() {
            Frame::none()
                .fill(FOND_CARTE)
                .stroke(Stroke::new(1.5, Color32::from_rgb(100, 80, 35)))
                .rounding(8.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("{} souhaite acheter :", offre.club_acheteur))
                                    .color(Color32::GRAY)
                                    .font(FontId::proportional(12.0)),
                            );
                            ui.label(
                                RichText::new(&offre.joueur_nom)
                                    .font(FontId::proportional(17.0))
                                    .color(Color32::WHITE)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(format!("Offre : {}", fmt_eur(offre.montant_eur)))
                                    .color(OR)
                                    .font(FontId::proportional(14.0)),
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let btn_refus = egui::Button::new(
                                RichText::new("✗  Refuser")
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(13.0)),
                            )
                            .fill(ROUGE_VIF)
                            .stroke(Stroke::NONE)
                            .rounding(6.0);
                            if ui.add(btn_refus).clicked() {
                                action = Some((i, false));
                            }
                            ui.add_space(8.0);
                            let btn_accept = egui::Button::new(
                                RichText::new("✓  Accepter")
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(13.0)),
                            )
                            .fill(VERT)
                            .stroke(Stroke::NONE)
                            .rounding(6.0);
                            if ui.add(btn_accept).clicked() {
                                action = Some((i, true));
                            }
                        });
                    });
                });
            ui.add_space(4.0);
        }
    });

    if let Some((idx, accepter)) = action {
        let offre = etat.offres_recues.remove(idx);
        if accepter {
            equipe.budget_eur += offre.montant_eur;
            etat.message = Some(format!(
                "✓ {} vendu à {} pour {} — budget : {}",
                offre.joueur_nom, offre.club_acheteur,
                fmt_eur(offre.montant_eur), fmt_eur(equipe.budget_eur)
            ));
        } else {
            etat.message = Some(format!(
                "Offre de {} pour {} refusée.",
                offre.club_acheteur, offre.joueur_nom
            ));
        }
    }
}

// ──────────────────────────────────────────────────────────
// Modale unique : recrutement direct (libre) ou offre (club)
// ──────────────────────────────────────────────────────────

fn render_modal(ctx: &Context, equipe: &mut Club, etat: &mut EtatMercato) {
    let idx = match etat.joueur_selectionne {
        Some(i) => i,
        None => return,
    };
    let joueur = etat.tous_joueurs[idx].clone();
    let est_libre = joueur.club_nom.is_none();

    let titre = if est_libre { "Recrutement" } else { "Faire une offre" };
    let mut open = true;

    egui::Window::new(titre)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .min_width(360.0)
        .open(&mut open)
        .show(ctx, |ui| {
            ui.add_space(6.0);

            if est_libre {
                // ── Joueur libre : prix fixe = valeur marché ──
                let cout = joueur.valeur_marche_eur;
                let peut_payer = equipe.budget_eur >= cout;

                ui.label(
                    RichText::new(format!("Recruter {} ?", joueur.nom))
                        .font(FontId::proportional(17.0))
                        .strong(),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("Indemnité de transfert : {}", fmt_eur(cout)))
                        .color(OR),
                );
                ui.label(format!("Salaire : {}/semaine", fmt_eur(joueur.salaire_semaine_eur)));
                ui.label(
                    RichText::new(format!("Budget disponible : {}", fmt_eur(equipe.budget_eur)))
                        .color(VERT)
                        .font(FontId::proportional(13.0)),
                );
                if !peut_payer {
                    ui.label(
                        RichText::new("⚠ Budget insuffisant")
                            .color(ROUGE_VIF)
                            .font(FontId::proportional(12.0)),
                    );
                }
                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    let ok = egui::Button::new(RichText::new("✓  Confirmer").color(Color32::WHITE))
                        .fill(if peut_payer { VERT } else { Color32::from_rgb(60, 60, 80) })
                        .stroke(Stroke::NONE)
                        .rounding(6.0);
                    if ui.add_enabled(peut_payer, ok).clicked() {
                        equipe.budget_eur -= cout;
                        etat.message = Some(format!(
                            "✓ {} recruté pour {} — budget restant : {}",
                            joueur.nom, fmt_eur(cout), fmt_eur(equipe.budget_eur)
                        ));
                        etat.joueur_selectionne = None;
                    }
                    ui.add_space(8.0);
                    if ui.button("Annuler").clicked() {
                        etat.joueur_selectionne = None;
                    }
                });
            } else {
                // ── Joueur sous contrat : offre de transfert ──
                let club_nom = joueur.club_nom.as_deref().unwrap_or("");
                let valeur = joueur.valeur_marche_eur as f64;

                ui.label(
                    RichText::new(format!("Offre pour {} ({})", joueur.nom, club_nom))
                        .font(FontId::proportional(16.0))
                        .strong(),
                );
                ui.label(
                    RichText::new(format!("Valeur estimée : {}", fmt_eur(joueur.valeur_marche_eur)))
                        .color(Color32::GRAY)
                        .font(FontId::proportional(13.0)),
                );
                ui.label(
                    RichText::new(format!("Budget disponible : {}", fmt_eur(equipe.budget_eur)))
                        .color(VERT)
                        .font(FontId::proportional(13.0)),
                );
                ui.add_space(10.0);
                ui.label("Montant de votre offre :");
                ui.add(
                    egui::Slider::new(&mut etat.offre_montant, (valeur * 0.5)..=(valeur * 2.0))
                        .custom_formatter(|v, _| fmt_eur(v as i64)),
                );

                // Avertissement si budget insuffisant
                if etat.offre_montant as i64 > equipe.budget_eur {
                    ui.label(
                        RichText::new("⚠ Budget insuffisant pour cette offre")
                            .color(ROUGE_VIF)
                            .font(FontId::proportional(12.0)),
                    );
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    let peut_payer = etat.offre_montant as i64 <= equipe.budget_eur;
                    let btn_envoyer = egui::Button::new(
                        RichText::new("Envoyer l'offre").color(Color32::WHITE),
                    )
                    .fill(if peut_payer { BLEU } else { Color32::from_rgb(60, 60, 80) })
                    .stroke(Stroke::NONE)
                    .rounding(6.0);

                    if ui.add_enabled(peut_payer, btn_envoyer).clicked() {
                        let seuil = if joueur.reputation > 90 { 1.15 } else if joueur.reputation > 80 { 1.0 } else { 0.85 };
                        let montant = etat.offre_montant as i64;
                        if etat.offre_montant >= valeur * seuil {
                            equipe.budget_eur -= montant;
                            etat.message = Some(format!(
                                "✓ {} rejoint votre club pour {} — budget restant : {}",
                                joueur.nom, fmt_eur(montant), fmt_eur(equipe.budget_eur)
                            ));
                        } else {
                            etat.message = Some(format!(
                                "✗ {} a refusé votre offre de {}.",
                                club_nom, fmt_eur(montant)
                            ));
                        }
                        etat.joueur_selectionne = None;
                    }
                    ui.add_space(8.0);
                    if ui.button("Annuler").clicked() {
                        etat.joueur_selectionne = None;
                    }
                });
            }
            ui.add_space(6.0);
        });

    if !open {
        etat.joueur_selectionne = None;
    }
}
