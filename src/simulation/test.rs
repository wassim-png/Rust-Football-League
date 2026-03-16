use rand::Rng;

const COEF_NOTE_GENERALE: f32 = 0.5;
const COEF_NOTE_COLLECTIF: f32 = 0.3;
const COEF_FORME_GENERALE: f32 = 0.2;
const COEF_REGULATEUR: f32 = 1.0;

fn calculer_note_globale(equipe: &CompositionMatch) -> f32 {
    COEF_FORME_GENERALE * equipe.forme_generale
        + COEF_NOTE_GENERALE * equipe.note_generale
        + COEF_NOTE_COLLECTIF * equipe.note_collectif
}

fn calcul_cote_match(equipe1: &CompositionMatch, equipe2: &CompositionMatch) -> (f32, f32) {
    let res = calculer_note_globale(equipe1) - calculer_note_globale(equipe2);

    let coef_eq1 = (50.0 + res * COEF_REGULATEUR).clamp(5.0, 95.0);
    let coef_eq2 = 100.0 - coef_eq1;

    (coef_eq1, coef_eq2)
}

fn calcul_occasions(cote: f32) -> i32 {
    if cote < 40.0 {
        3
    } else if cote < 50.0 {
        4
    } else if cote < 60.0 {
        5
    } else {
        6
    }
}

fn calcul_proba_conversion(finition: f32) -> f32 {
    (5.0 + finition * 0.2).clamp(5.0, 30.0)
}

fn simuler_score(equipe1: &CompositionMatch, equipe2: &CompositionMatch) -> (i32, i32) {
    let (cote1, cote2) = calcul_cote_match(equipe1, equipe2);

    let occasions1 = calcul_occasions(cote1);
    let occasions2 = calcul_occasions(cote2);

    let proba1 = calcul_proba_conversion(equipe1.finition);
    let proba2 = calcul_proba_conversion(equipe2.finition);

    let mut buts1 = 0;
    let mut buts2 = 0;

    let mut rng = rand::thread_rng();

    for _ in 0..occasions1 {
        let x = rng.gen_range(0.0..100.0);
        if x < proba1 {
            buts1 += 1;
        }
    }

    for _ in 0..occasions2 {
        let x = rng.gen_range(0.0..100.0);
        if x < proba2 {
            buts2 += 1;
        }
    }

    (buts1, buts2)
}

fn determiner_vainqueur<'a>(
    equipe1: &'a CompositionMatch,
    equipe2: &'a CompositionMatch,
    buts1: i32,
    buts2: i32,
) -> Option<&'a CompositionMatch> {
    if buts1 > buts2 {
        Some(equipe1)
    } else if buts2 > buts1 {
        Some(equipe2)
    } else {
        None
    }
}