#[derive(Debug, Clone)]
pub struct SlotComposition {
    pub cle: String,
    pub poste_attendu: Poste,
    pub joueur_selectionne: Option<JoueurComposition>,
}