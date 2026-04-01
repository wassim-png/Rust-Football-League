use crate::models::Joueur;
use rusqlite::Result;

#[allow(dead_code)]
pub trait JoueurDAO {
    fn get_joueurs_by_club_id(&self, club_id: i32) -> Result<Vec<Joueur>>;
    fn update_forme_joueur(&self, joueur_id : i32, forme : i32) -> Result<(), String>;
    fn recuperation_forme_globale(&self, joueurs_exclus: &[i32]) -> Result<(), String>;
}
