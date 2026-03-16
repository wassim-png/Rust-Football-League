pub enum Ecran {
    Selection,
    MenuPrincipal,
    Composition,
    InfosClub,
    DetailsJoueur,
}

// Pour pouvoir copier l'équipe choisie dans l'équipe actuel
#[derive(Clone)]
pub struct Equipe {
    pub id: i32,
    pub nom: String,
    pub stade: String,
    pub budget: i32,
}