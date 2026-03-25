pub enum Ecran {
    Accueil,
    Selection,
    MenuPrincipal,
    Composition,
    InfosClub,
    DetailsJoueur,
    Mercato,
}

#[derive(PartialEq, Clone)]
pub enum OngletMercato {
    JoueursDisponibles,
    OffresRecues,
    MesJoueurs,
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
    pub url_stade: String,
}

#[derive(Debug, Clone)]
pub struct Joueur {
    pub id: i32,
    pub nom: String,
    pub age: i32,
    pub poste: String,
    pub reputation: i32,
    pub valeur_marche_eur: i64,
    pub salaire_semaine_eur: i64,
    /// None = joueur libre, Some(nom) = joueur sous contrat
    pub club_nom: Option<String>,
}

#[derive(Clone)]
pub struct OffreTransfert {
    pub joueur_id: i32,
    pub joueur_nom: String,
    pub club_acheteur_id: i32,
    pub club_acheteur: String,
    pub montant_eur: i64,
}

pub struct EtatMercato {
    pub onglet: OngletMercato,
    pub tous_joueurs: Vec<Joueur>,
    pub mes_joueurs: Vec<Joueur>,
    pub offres_recues: Vec<OffreTransfert>,
    pub donnees_chargees: bool,
    pub recherche: String,
    /// None = tous les postes, Some("ATTAQUE") etc.
    pub filtre_poste: Option<String>,
    pub message: Option<String>,
    /// Index dans tous_joueurs du joueur sélectionné pour recrutement/offre
    pub joueur_selectionne: Option<usize>,
    pub offre_montant: f64,
    /// (joueur_id, club_id) à persister en DB après recrutement/achat
    pub action_recrutement: Option<(i32, i32)>,
    /// (joueur_id, Option<nouveau_club_id>) — None = libéré sur le marché
    pub action_vente: Option<(i32, Option<i32>)>,
}

impl Default for EtatMercato {
    fn default() -> Self {
        Self {
            onglet: OngletMercato::JoueursDisponibles,
            tous_joueurs: vec![],
            mes_joueurs: vec![],
            offres_recues: vec![],
            donnees_chargees: false,
            recherche: String::new(),
            filtre_poste: None,
            message: None,
            joueur_selectionne: None,
            offre_montant: 0.0,
            action_recrutement: None,
            action_vente: None,
        }
    }
}
