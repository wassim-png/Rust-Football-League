#[derive(Debug, Clone)]
pub struct Match {
    pub id: i32,
    pub saison_id: i32,
    pub journee: i32,
    pub club_domicile_id: i32,
    pub club_exterieur_id: i32,
    pub date_coup_envoi: Option<String>,
}