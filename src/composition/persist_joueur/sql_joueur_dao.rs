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
            -- Note : On choisit la table selon le poste
            CASE 
                WHEN j.poste = 'GARDIEN' THEN IFNULL(g.note_actuelle, j.reputation)
                ELSE IFNULL(a.note_actuelle, j.reputation)
            END as note,
            -- Forme : Si NULL dans la bonne table, on force 100
            CASE 
                WHEN j.poste = 'GARDIEN' THEN IFNULL(g.forme, 100)
                ELSE IFNULL(a.forme, 100)
            END as forme,
            -- Nationalité : 'FR' par défaut
            CASE 
                WHEN j.poste = 'GARDIEN' THEN IFNULL(g.nationalite, 'FR')
                ELSE IFNULL(a.nationalite, 'FR')
            END as nationalite,
            j.valeur_marche_eur,
            j.salaire_semaine_eur,
            c.nom
        FROM joueurs j
        JOIN clubs c ON j.club_id = c.id
        LEFT JOIN attributs_joueur_saison a ON a.joueur_id = j.id
        LEFT JOIN attributs_gardien_saison g ON g.joueur_id = j.id
        WHERE j.club_id = ?"
                
        )?;

        let joueur_iter = stmt.query_map([club_id], |row| {
           let nom_joueur: String = row.get(1)?; 
    // 2. On récupère la valeur brute de la forme
    let forme_brute: rusqlite::types::Value = row.get(6)?; 
    
    // 3. On affiche la vérité !
    println!("SQL DEBUG -> Joueur: {} | Valeur brute colonne 6 (forme): {:?}", nom_joueur, forme_brute);
            Ok(Joueur {
                id: row.get(0)?,
                nom: row.get(1)?,
                age: row.get(2)?,
                poste: row.get(3)?,
                reputation: row.get(4)?,
                note_actuelle: row.get(5)?,
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


  fn update_forme_joueur(&self, joueur_id: i32, forme: i32) -> Result<(), String> {
    
    self.conn.execute(
        "UPDATE attributs_joueur_saison 
         SET forme = ?1 
         WHERE joueur_id = ?2",
        rusqlite::params![
            forme,      // ?1
            joueur_id   // ?2
        ],
    ).map_err(|e| format!("Erreur lors de la MAJ de la forme (joueur de champ) : {}", e))?; 

    
    self.conn.execute(
        "UPDATE attributs_gardien_saison 
         SET forme = ?1 
         WHERE joueur_id = ?2",
        rusqlite::params![
            forme,      // ?1
            joueur_id   // ?2
        ],
    ).map_err(|e| format!("Erreur lors de la MAJ de la forme (gardien) : {}", e))?; 

    Ok(())
}

    fn recuperation_forme_globale(&self, joueurs_exclus: &[i32]) -> Result<(), String> {
        if joueurs_exclus.is_empty() {
            self.conn.execute(
                "UPDATE attributs_joueur_saison SET forme = MIN(100, IFNULL(forme, 100) + 15)",
                [],
            ).map_err(|e| format!("Erreur recup forme joueurs champ: {}", e))?;

            self.conn.execute(
                "UPDATE attributs_gardien_saison SET forme = MIN(100, IFNULL(forme, 100) + 15)",
                [],
            ).map_err(|e| format!("Erreur recup forme gardiens: {}", e))?;
        } else {
            let in_clause = joueurs_exclus
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let stmt_joueur = format!(
                "UPDATE attributs_joueur_saison SET forme = MIN(100, IFNULL(forme, 100) + 15) WHERE joueur_id NOT IN ({})",
                in_clause
            );
            self.conn.execute(&stmt_joueur, []).map_err(|e| format!("Erreur recup forme joueurs champ avec exclusions: {}", e))?;

            let stmt_gardien = format!(
                "UPDATE attributs_gardien_saison SET forme = MIN(100, IFNULL(forme, 100) + 15) WHERE joueur_id NOT IN ({})",
                in_clause
            );
            self.conn.execute(&stmt_gardien, []).map_err(|e| format!("Erreur recup forme gardiens avec exclusions: {}", e))?;
        }

        Ok(())
    }
}