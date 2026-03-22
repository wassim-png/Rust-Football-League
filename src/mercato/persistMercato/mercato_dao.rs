use rusqlite::Result;
use crate::models::{Joueur, OffreTransfert};

pub trait MercatoDAO {
    fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>>;
    fn get_joueurs_mon_club(&self, mon_club_id: i32) -> Result<Vec<Joueur>>;
    fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>>;
}
