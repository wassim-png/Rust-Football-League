#[allow(dead_code)]
pub struct CompositionRules;

#[allow(dead_code)]
impl CompositionRules {
    // -------- COLLECTIF --------
    pub const BASE_COLLECTIF: f32 = 50.0;
    pub const BONUS_COHESION_PAR_JOUEUR: f32 = 4.0;
    pub const BONUS_FRANCAIS: f32 = 2.0;
    pub const MAX_COLLECTIF: f32 = 100.0;

    // -------- MALUS --------
    pub const MALUS_HORS_POSTE: f32 = 0.20;

    // -------- FINITION (pondérations) --------
    pub const POIDS_GARDIEN: f32 = 0.05;
    pub const POIDS_DEFENSE: f32 = 0.3;
    pub const POIDS_MILIEU: f32 = 0.7;
    pub const POIDS_ATTAQUE: f32 = 1.0;
}