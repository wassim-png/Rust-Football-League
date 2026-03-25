use std::sync::Arc;
use rusqlite::{Connection, Result, params};
use crate::models::Match;
use super::calendrier_dao::CalendrierDAO;

pub struct SqlCalendrierDAO {
    pub conn: Arc<Connection>,
}

impl SqlCalendrierDAO {
    /// Retourne "16 Août 2025" pour journée 1, +7 jours par journée
    fn date_journee(journee: i32) -> String {
        let mois_noms = [
            "Janvier", "Février", "Mars", "Avril", "Mai", "Juin",
            "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre",
        ];
        // J1 = 16 Août 2025 = jour 228 de 2025
        let total_doy = 228u32 + (journee - 1) as u32 * 7;

        let (year, doy) = if total_doy <= 365 {
            (2025u32, total_doy)
        } else {
            (2026u32, total_doy - 365)
        };

        let month_days: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut remaining = doy;
        let mut month_idx = 11usize;
        for (i, &days) in month_days.iter().enumerate() {
            if remaining <= days {
                month_idx = i;
                break;
            }
            remaining -= days;
        }

        format!("{} {} {}", remaining, mois_noms[month_idx], year)
    }

    /// Round-robin classique : 18 clubs → 34 journées, 9 matchs/journée
    /// Retourne Vec<(journee, home_id, away_id, date_str)>
    fn generer_matchs_round_robin(clubs: &[i32]) -> Vec<(i32, i32, i32, String)> {
        let n = clubs.len(); // 18
        let fixed = clubs[0];
        let mut rotatable: Vec<i32> = clubs[1..].to_vec();
        let mut result = Vec::new();

        for round in 0..(n - 1) {
            let journee = (round + 1) as i32;
            let date = Self::date_journee(journee);

            // Premier match : fixed vs rotatable[0], on alterne dom/ext selon la ronde
            if round % 2 == 0 {
                result.push((journee, fixed, rotatable[0], date.clone()));
            } else {
                result.push((journee, rotatable[0], fixed, date.clone()));
            }

            // Paires restantes : outside-in
            for i in 0..(n / 2 - 1) {
                let a = rotatable[i + 1];
                let b = rotatable[n - 2 - i];
                if round % 2 == 0 {
                    result.push((journee, a, b, date.clone()));
                } else {
                    result.push((journee, b, a, date.clone()));
                }
            }

            // Rotation à droite
            rotatable.rotate_right(1);
        }

        // Retour : swap dom/ext, journée += n-1
        let aller: Vec<_> = result.clone();
        for (j, home, away, _) in aller {
            let journee_retour = j + (n as i32 - 1);
            let date_retour = Self::date_journee(journee_retour);
            result.push((journee_retour, away, home, date_retour));
        }

        result
    }
}

impl CalendrierDAO for SqlCalendrierDAO {
    fn calendrier_existe(&self, saison_id: i32) -> Result<bool> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM matchs WHERE saison_id = ?1",
            params![saison_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    fn generer_calendrier(&self, saison_id: i32, club_ids: Vec<i32>) -> Result<()> {
        let matchs = Self::generer_matchs_round_robin(&club_ids);
        for (journee, home_id, away_id, date) in matchs {
            self.conn.execute(
                "INSERT INTO matchs (saison_id, journee, club_domicile_id, club_exterieur_id, date_coup_envoi)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![saison_id, journee, home_id, away_id, date],
            )?;
        }
        Ok(())
    }

    fn get_tous_matchs(&self, saison_id: i32) -> Result<Vec<Match>> {
        let mut stmt = self.conn.prepare(
            "SELECT m.id, m.journee,
                    m.club_domicile_id,  cd.nom,  id_info.url_logo,
                    m.club_exterieur_id, ce.nom,  ie_info.url_logo,
                    m.date_coup_envoi,
                    r.buts_domicile, r.buts_exterieur
             FROM matchs m
             JOIN clubs cd       ON cd.id       = m.club_domicile_id
             JOIN clubs ce       ON ce.id       = m.club_exterieur_id
             JOIN info_club id_info ON id_info.club_id = m.club_domicile_id
             JOIN info_club ie_info ON ie_info.club_id = m.club_exterieur_id
             LEFT JOIN resultats_matchs r ON r.match_id = m.id
             WHERE m.saison_id = ?1
             ORDER BY m.journee, m.id",
        )?;

        let iter = stmt.query_map(params![saison_id], |row| {
            Ok(Match {
                id: row.get(0)?,
                journee: row.get(1)?,
                club_domicile_id: row.get(2)?,
                club_domicile_nom: row.get(3)?,
                club_domicile_logo: row.get(4)?,
                club_exterieur_id: row.get(5)?,
                club_exterieur_nom: row.get(6)?,
                club_exterieur_logo: row.get(7)?,
                date_coup_envoi: row.get(8)?,
                buts_domicile: row.get(9)?,
                buts_exterieur: row.get(10)?,
            })
        })?;

        let mut matchs = Vec::new();
        for m in iter {
            matchs.push(m?);
        }
        Ok(matchs)
    }
}
