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
            j.valeur_marche_eur,
            j.salaire_semaine_eur,
            c.nom AS club_nom,
            COALESCE(aj.note_actuelle, ag.note_actuelle) AS note_actuelle,
            COALESCE(aj.forme, ag.forme) AS forme,
            COALESCE(aj.nationalite, ag.nationalite) AS nationalite
        FROM joueurs j
        JOIN clubs c ON j.club_id = c.id
        LEFT JOIN attributs_joueurs aj ON aj.joueur_id = j.id
        LEFT JOIN attributs_gardiens ag ON ag.joueur_id = j.id
        WHERE j.club_id = ?1
        ORDER BY
            CASE j.poste
                WHEN 'GARDIEN' THEN 1
                WHEN 'DEFENSE' THEN 2
                WHEN 'MILIEU' THEN 3
                WHEN 'ATTAQUE' THEN 4
                ELSE 5
            END,
            j.reputation DESC
        "
    )?;

        let joueur_iter = stmt.query_map([club_id], |row| {
            Ok(Joueur {
                id: row.get(0)?,
                nom: row.get(1)?,
                age: row.get(2)?,
                poste: row.get(3)?,
                reputation: row.get(4)?,
                valeur_marche_eur: row.get(5)?,
                salaire_semaine_eur: row.get(6)?,
                club_nom: row.get(7)?,
            })
        })?;

        let mut joueurs = Vec::new();
        for joueur in joueur_iter {
            joueurs.push(joueur?);
        }

        Ok(joueurs)
    }
}