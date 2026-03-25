
mod models;
mod app;
mod selection_club;
mod infos_club;
pub mod page;
mod database;
mod mercato;
mod calendrier;
use database:: Database;


use app::MyApp;
fn main() -> eframe::Result<()> {
    // Initialize the database before starting the GUI
   let db = Database::new("db/simulation.db")
   .expect("Erreur fatale : Impossible d'initialiser la base de données");

    let options = eframe::NativeOptions::default();
    eframe::run_native("Rust Football League", options, 
    Box::new(move |cc| {  
        egui_extras::install_image_loaders(&cc.egui_ctx); 

        Box::new(MyApp::new(db.conn.clone()))
    }),
)
}