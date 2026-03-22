use std::sync::Arc;
use rusqlite::{Connection, Result};
use rand::{seq::SliceRandom, Rng};
use crate::models::{Joueur, OffreTransfert};
use crate::mercato::persistMercato::mercato_dao::MercatoDAO;

pub struct SqlMercatoDAO {
    pub conn: Arc<Connection>,
}

impl MercatoDAO for SqlMercatoDAO {
    /// Joueurs disponibles = libres (club_id IS NULL) + joueurs des autres clubs
    fn get_tous_joueurs_disponibles(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        let mut stmt = self.conn.prepare(
            "SELECT j.id, j.nom, j.age, j.poste, j.reputation,
                    j.valeur_marche_eur, j.salaire_semaine_eur,
                    c.nom as club_nom
             FROM joueurs j
             LEFT JOIN clubs c ON c.id = j.club_id
             WHERE j.club_id IS NULL OR j.club_id != ?1
             ORDER BY j.valeur_marche_eur DESC",
        )?;
        let iter = stmt.query_map([mon_club_id], |row| {
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
        iter.collect()
    }

    /// Joueurs de notre propre club (pour générer des offres IA)
    fn get_joueurs_mon_club(&self, mon_club_id: i32) -> Result<Vec<Joueur>> {
        let mut stmt = self.conn.prepare(
            "SELECT j.id, j.nom, j.age, j.poste, j.reputation,
                    j.valeur_marche_eur, j.salaire_semaine_eur, c.nom
             FROM joueurs j
             INNER JOIN clubs c ON c.id = j.club_id
             WHERE j.club_id = ?1",
        )?;
        let iter = stmt.query_map([mon_club_id], |row| {
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
        iter.collect()
    }

    /// Génère 1 à 3 offres aléatoires de clubs adverses pour nos joueurs
    fn generer_offres_ia(&self, mon_club_id: i32) -> Result<Vec<OffreTransfert>> {
        let mes_joueurs = self.get_joueurs_mon_club(mon_club_id)?;
        if mes_joueurs.is_empty() {
            return Ok(vec![]);
        }

        let mut stmt = self.conn.prepare(
            "SELECT nom FROM clubs WHERE id != ?1 ORDER BY id",
        )?;
        let autres_clubs: Vec<String> = stmt
            .query_map([mon_club_id], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok())
            .collect();

        if autres_clubs.is_empty() {
            return Ok(vec![]);
        }

        let mut rng = rand::thread_rng();
        let nb_offres = rng.gen_range(1..=3).min(mes_joueurs.len());
        let joueurs_cibles: Vec<&Joueur> = mes_joueurs.choose_multiple(&mut rng, nb_offres).collect();

        let offres = joueurs_cibles
            .iter()
            .filter_map(|j| {
                let club = autres_clubs.choose(&mut rng)?;
                let multiplicateur = rng.gen_range(0.90_f64..1.30_f64);
                Some(OffreTransfert {
                    joueur_nom: j.nom.clone(),
                    club_acheteur: club.clone(),
                    montant_eur: (j.valeur_marche_eur as f64 * multiplicateur) as i64,
                })
            })
            .collect();

        Ok(offres)
    }
}
