#[derive(Debug, Clone)]
pub struct ResultatSimulationMatch {
    pub match_id: i32,
    pub buts_domicile: i32,
    pub buts_exterieur: i32,
    pub vainqueur_id: Option<i32>,
}