use std::io::{self, Write};

use crate::controller::match_controller::MatchController;

pub struct MatchFrame {
    controller: MatchController,
}

impl MatchFrame {
    pub fn new() -> Self {
        Self {
            controller: MatchController::new(),
        }
    }

    pub fn afficher_menu(&self) {
        println!("==============================");
        println!("     FOOTBALL MANAGER");
        println!("==============================");
        println!("1. Simuler un match");
        println!("0. Quitter");
        println!("==============================");
    }

    pub fn lancer(&self) {
        loop {
            self.afficher_menu();

            let choix = Self::lire_entier("Choisis une option : ");

            match choix {
                1 => self.simuler_match_ui(),
                0 => {
                    println!("Fermeture de l'application.");
                    break;
                }
                _ => {
                    println!("Option invalide.\n");
                }
            }
        }
    }

    fn simuler_match_ui(&self) {
        let match_id = Self::lire_entier("Entre l'id du match à simuler : ");

        match self.controller.simuler_match(match_id) {
            Ok(resultat) => {
                println!("\n===== RESULTAT DU MATCH =====");
                println!("Match ID       : {}", resultat.match_id);
                println!("Buts domicile  : {}", resultat.buts_domicile);
                println!("Buts exterieur : {}", resultat.buts_exterieur);

                match resultat.vainqueur_id {
                    Some(id) => println!("Vainqueur club_id : {}", id),
                    None => println!("Match nul"),
                }

                println!("=============================\n");
            }
            Err(e) => {
                println!("\nErreur lors de la simulation : {}\n", e);
            }
        }
    }

    fn lire_entier(message: &str) -> i32 {
        loop {
            print!("{}", message);
            io::stdout().flush().expect("Impossible de flush stdout");

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse::<i32>() {
                    Ok(valeur) => return valeur,
                    Err(_) => {
                        println!("Veuillez entrer un entier valide.");
                    }
                },
                Err(_) => {
                    println!("Erreur de lecture, veuillez réessayer.");
                }
            }
        }
    }
}
