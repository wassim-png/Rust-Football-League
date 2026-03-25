use rusqlite::params;

use crate::dao::composition_dao::CompositionDao;
use crate::db::db_connection::DbConnection;
use crate::model::composition_match::CompositionMatch;

pub struct SqliteCompositionDao;

impl SqliteCompositionDao {
    pub fn new() -> Self {
        Self
    }
}

impl CompositionDao for SqliteCompositionDao {
    fn find_by_match_and_club(
        &self,
        match_id: i32,
        club_id: i32,
        saison_id: i32,
    ) -> Result<Option<CompositionMatch>, String> {
        let connection = DbConnection::get_connection()
            .map_err(|e| format!("Erreur connexion SQLite : {}", e))?;

        let mut stmt = connection
            .prepare(
                "
                SELECT
                    cm.match_id,
                    j.club_id,
                    AVG(COALESCE(ajs.note_actuelle, ags.note_actuelle)) as note_generale,
                    AVG(COALESCE(ajs.moral, ags.moral)) as note_collectif,
                    AVG(COALESCE(ajs.forme, ags.forme)) as forme_generale,
                    AVG(
                        CASE
                            WHEN j.poste = 'GARDIEN' THEN 0
                            ELSE COALESCE(ajs.tir, 0)
                        END
                    ) as finition
                FROM compositions_match cm
                JOIN joueurs j
                    ON j.id = cm.joueur_id
                LEFT JOIN attributs_joueur_saison ajs
                    ON ajs.joueur_id = j.id AND ajs.saison_id = ?
                LEFT JOIN attributs_gardien_saison ags
                    ON ags.joueur_id = j.id AND ags.saison_id = ?
                WHERE cm.match_id = ?
                  AND j.club_id = ?
                  AND cm.est_titulaire = 1
                GROUP BY cm.match_id, j.club_id
                ",
            )
            .map_err(|e| format!("Erreur préparation find_by_match_and_club : {}", e))?;

        let mut rows = stmt
            .query(params![saison_id, saison_id, match_id, club_id])
            .map_err(|e| format!("Erreur exécution find_by_match_and_club : {}", e))?;

        match rows.next().map_err(|e| format!("Erreur lecture composition : {}", e))? {
            Some(row) => Ok(Some(CompositionMatch {
                match_id: row.get(0).map_err(|e| format!("Erreur match_id : {}", e))?,
                club_id: row.get(1).map_err(|e| format!("Erreur club_id : {}", e))?,
                note_generale: row.get(2).map_err(|e| format!("Erreur note_generale : {}", e))?,
                note_collectif: row.get(3).map_err(|e| format!("Erreur note_collectif : {}", e))?,
                forme_generale: row.get(4).map_err(|e| format!("Erreur forme_generale : {}", e))?,
                finition: row.get(5).map_err(|e| format!("Erreur finition : {}", e))?,
            })),
            None => Ok(None),
        }
    }
}