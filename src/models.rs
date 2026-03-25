pub enum Ecran {
    Accueil,
    Selection,
    MenuPrincipal,
    Composition,
    InfosClub,
    DetailsJoueur,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Joueur {
    pub id: i32,
    pub club_id: i32,
    pub nom: String,
    pub age: i32,
    pub numero: i32,
    pub poste: String,
    pub note_actuelle: i32,
    pub forme: i32,
    pub nationalite: String,
}