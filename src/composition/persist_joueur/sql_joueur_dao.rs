use std::sync::Arc;
use rusqlite::{Connection, Result, Row};
use crate::models::Joueur;
use crate::composition::persist_joueur::joueur_dao::JoueurDAO;

pub struct SqliteJoueurDAO {
    pub conn: Arc<Connection>,
}

impl JoueurDAO for SqliteJoueurDAO {
    fn get_joueurs_by_club_id(&self, club_id: i32) -> Result<Vec<Joueur>> {
        let mut stmt = self.conn.prepare(
            "SELECT j.id, j.club_id, j.nom, j.age, j.numero, j.poste,
                    COALESCE(a.note_actuelle, g.note_actuelle, 50) AS note,
                    COALESCE(a.forme, g.forme, 100) AS forme,
                    COALESCE(a.nationalite, g.nationalite, '') AS nationalite
             FROM joueurs j
             LEFT JOIN attributs_joueur_saison a ON a.joueur_id = j.id AND j.poste <> 'GARDIEN'
             LEFT JOIN attributs_gardien_saison g ON g.joueur_id = j.id AND j.poste = 'GARDIEN'
             WHERE j.club_id = ?
             ORDER BY 
                CASE j.poste 
                    WHEN 'GARDIEN' THEN 1
                    WHEN 'DEFENSE' THEN 2
                    WHEN 'MILIEU' THEN 3
                    WHEN 'ATTAQUE' THEN 4
                    ELSE 5
                END,
                note DESC"
        )?;

        let joueur_iter = stmt.query_map([club_id], |row: &Row| {
            Ok(Joueur {
                id: row.get(0)?,
                club_id: row.get(1)?,
                nom: row.get(2)?,
                age: row.get(3)?,
                numero: row.get(4)?,
                poste: row.get(5)?,
                note_actuelle: row.get(6)?,
                forme: row.get(7)?,
                nationalite: row.get(8)?,
            })
        })?;

        let mut joueurs = Vec::new();
        for joueur in joueur_iter {
            joueurs.push(joueur?);
        }

        Ok(joueurs)
    }
}
