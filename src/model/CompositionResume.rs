#[derive(Debug, Clone)]
pub struct CompositionResume {
    pub note_generale: f32,
    pub note_collectif: f32,
    pub forme_generale: f32,
    pub finition: f32,
    pub nb_joueurs_selectionnes: usize,
    pub composition_complete: bool,
}