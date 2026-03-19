
mod models; // Déclare l'existence de models.rs
mod app;
mod selection_club;
mod database;
use database:: Database;
use std::sync::Arc;

use app::MyApp;
fn main() -> eframe::Result<()> {
    // Initialize the database before starting the GUI
   let db = Database::new("db/simulation.db")
   .expect("Erreur fatale : Impossible d'initialiser la base de données");

    let options = eframe::NativeOptions::default();
    eframe::run_native("Foot Manager Rust", options, Box::new(move |_cc| Box::new(MyApp::new(db.conn.clone()))))
}