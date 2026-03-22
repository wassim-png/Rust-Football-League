
mod models; // Déclare l'existence de models.rs
mod app;
mod selection_club;
pub mod page;
mod database;
use database:: Database;
use std::sync::Arc;


use app::MyApp;
fn main() -> eframe::Result<()> {
    // Initialize the database before starting the GUI
   let db = Database::new("db/simulation.db")
   .expect("Erreur fatale : Impossible d'initialiser la base de données");

    let options = eframe::NativeOptions::default();
    eframe::run_native("Rust Football League", options, 
    Box::new(move |cc| {
        // AJOUTE CETTE LIGNE ABSOLUMENT :
        egui_extras::install_image_loaders(&cc.egui_ctx); 

        Box::new(MyApp::new(db.conn.clone()))
    }),
)
}