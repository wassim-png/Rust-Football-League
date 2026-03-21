pub enum Ecran {
    Accueil,
    Selection,
    MenuPrincipal,
    Composition,
    InfosClub,
    DetailsJoueur,
}

#[derive(Debug, Clone)]
pub struct Club {
    pub id: Option<i32>,  
    pub nom: String,        
    pub nom_court: String,
    pub reputation: i32,          
    pub budget_eur: i64,          
    pub revenu_par_journee_eur: i64,
    pub avantage_domicile: i32,    
}