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
    pub url_logo: String   
}


#[derive(Debug, Clone)]
pub struct InfosClub {
    pub club_id: Option<i32>,  
    pub nom : String,
    pub nom_stade: String,        
    pub stade_capacite: i32,
    pub reputation : i32,
    pub avantage_domicile : i32,
    pub revenu_par_journee_eur : i64,
    pub url_logo: String,          
    pub url_stade: String   
}

