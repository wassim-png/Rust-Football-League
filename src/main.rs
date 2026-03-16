
mod models; // Déclare l'existence de models.rs
mod app;
pub mod db_setup; // Module for DB initialization

use app::MyApp;
fn main() -> eframe::Result<()> {
    // Initialize the database before starting the GUI
    if let Err(e) = db_setup::init_db("db/simulation.db") {
        eprintln!("Failed to initialize database: {}", e);
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native("Foot Manager Rust", options, Box::new(|_cc| Box::new(MyApp::default())))
}