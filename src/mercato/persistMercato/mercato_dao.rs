use rusqlite::Result;
use crate::models::{Joueur, OffreTransfert};

pub trait MercatoDAO {
    fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>>;
    fn get_joueurs_mon_club(&self, mon_club_id: i32) -> Result<Vec<Joueur>>;
    fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>>;
    fn recruter_joueur(&self, joueur_id: i32, club_id: i32) -> Result<()>;
    /// None = libéré (joueur libre), Some(id) = transféré dans le club acheteur
    fn vendre_joueur(&self, joueur_id: i32, nouveau_club_id: Option<i32>) -> Result<()>;
    fn mettre_a_jour_budget_club(&self, club_id: i32, nouveau_budget: i64) -> Result<()>;
}
