use crate::models::Joueur;
use rusqlite::Result;

pub trait JoueurDAO {
    fn get_joueurs_by_club_id(&self, club_id: i32) -> Result<Vec<Joueur>>;
     fn update_forme_joueur(&self, joueur: &Joueur) -> Result<Vec<Joueur>>;
}
