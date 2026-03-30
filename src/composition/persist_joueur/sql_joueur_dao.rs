use std::sync::Arc;
use rusqlite::{Connection, Result};
use crate::models::Joueur;
use crate::composition::persist_joueur::joueur_dao::JoueurDAO;

pub struct SqliteJoueurDAO {
    pub conn: Arc<Connection>,
}

impl JoueurDAO for SqliteJoueurDAO {
    fn get_joueurs_by_club_id(&self, club_id: i32) -> Result<Vec<Joueur>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT
                j.id,
                j.nom,
                j.age,
                j.poste,
                j.reputation,
                COALESCE(a.note_actuelle, g.note_actuelle, j.reputation) as note,
                COALESCE(a.forme, g.forme, 100) as forme,
                COALESCE(a.nationalite, g.nationalite, 'FR') as nationalite,
                j.valeur_marche_eur,
                j.salaire_semaine_eur,
                c.nom
            FROM joueurs j
            JOIN clubs c ON j.club_id = c.id
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
                note DESC
            "
        )?;

        let joueur_iter = stmt.query_map([club_id], |row| {
            Ok(Joueur {
                id: row.get(0)?,
                nom: row.get(1)?,
                age: row.get(2)?,
                poste: row.get(3)?,
                reputation: row.get(4)?,
                note: row.get(5)?,
                forme: row.get(6)?,
                nationalite: row.get(7)?,
                valeur_marche_eur: row.get(8)?,
                salaire_semaine_eur: row.get(9)?,
                club_nom: row.get(10)?,
            })
        })?;

        let mut joueurs = Vec::new();
        for joueur in joueur_iter {
            joueurs.push(joueur?);
        }

        Ok(joueurs)
    }
}