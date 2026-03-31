pub struct MatchRules;

impl MatchRules {
    // -------- FORME APRES MATCH --------
    pub const PERTE_FORME_GARDIEN: f32 = 1.0;
    pub const PERTE_FORME_DEFENSE: f32 = 1.5;
    pub const PERTE_FORME_MILIEU: f32 = 2.0;
    pub const PERTE_FORME_ATTAQUE: f32 = 1.8;
    pub const FORME_MIN: f32 = 10.0;

    // -------- CLASSEMENT --------
    pub const POINTS_VICTOIRE: i32 = 3;
    pub const POINTS_NUL: i32 = 1;
    pub const POINTS_DEFAITE: i32 = 0;

    // -------- SIMULATION --------
    pub const COEF_NOTE_GENERALE: f32 = 0.5;
    pub const COEF_NOTE_COLLECTIF: f32 = 0.3;
    pub const COEF_FORME_GENERALE: f32 = 0.2;
    pub const COEF_REGULATEUR: f32 = 1.0;
    pub const COEF_REGULATEUR_BUT: f32 = 1.0;
}