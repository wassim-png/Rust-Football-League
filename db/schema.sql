-- ============================================================
-- Schéma SQLite "mini Football Manager" (FR) - version finale simplifiée
-- ============================================================

PRAGMA foreign_keys = ON;

-- (Optionnel) reset pour dev
-- DROP TABLE IF EXISTS evenement_matchs;
-- DROP TABLE IF EXISTS compositions_match;
-- DROP TABLE IF EXISTS transferts;
-- DROP TABLE IF EXISTS resultats_matchs;
-- DROP TABLE IF EXISTS matchs;
-- DROP TABLE IF EXISTS primes_classement_saison;
-- DROP TABLE IF EXISTS attributs_joueur_saison;
-- DROP TABLE IF EXISTS joueurs;
-- DROP TABLE IF EXISTS etat_club_saison;
-- DROP TABLE IF EXISTS saison_club;
-- DROP TABLE IF EXISTS clubs;
-- DROP TABLE IF EXISTS saisons;
-- DROP TABLE IF EXISTS competitions;

-- =========================
-- 1) Compétitions & Saisons
-- =========================

CREATE TABLE IF NOT EXISTS competitions (
  id               INTEGER PRIMARY KEY AUTOINCREMENT,
  nom              TEXT NOT NULL UNIQUE,
  type_competition TEXT NOT NULL CHECK (type_competition IN ('championnat', 'coupe')),

  -- Règles (simple)
  nb_equipes       INTEGER CHECK (nb_equipes IS NULL OR nb_equipes >= 2),
 

  -- Championnat uniquement (NULL autorisés si coupe)
  points_victoire  INTEGER CHECK (points_victoire IS NULL OR points_victoire >= 0),
  points_nul       INTEGER CHECK (points_nul IS NULL OR points_nul >= 0),
  points_defaite   INTEGER CHECK (points_defaite IS NULL OR points_defaite >= 0),

  CHECK (
    (type_competition <> 'championnat')
    OR (nb_equipes IS NOT NULL
        AND points_victoire IS NOT NULL
        AND points_nul IS NOT NULL
        AND points_defaite IS NOT NULL)
  )
);

CREATE TABLE IF NOT EXISTS saisons (
  id             INTEGER PRIMARY KEY AUTOINCREMENT,
  competition_id INTEGER NOT NULL,
  annee          INTEGER NOT NULL CHECK (annee BETWEEN 1900 AND 2100),

  FOREIGN KEY (competition_id) REFERENCES competitions(id) ON DELETE CASCADE,
  UNIQUE (competition_id, annee)
);

-- =========================
-- 2) Clubs & participation à une saison
-- =========================
CREATE TABLE IF NOT EXISTS clubs (
  id                     INTEGER PRIMARY KEY AUTOINCREMENT,
  nom                    TEXT NOT NULL UNIQUE,
  nom_court              TEXT NOT NULL UNIQUE,

  reputation             INTEGER NOT NULL DEFAULT 50 CHECK (reputation BETWEEN 0 AND 100),

  -- Argent (simple)
  budget_eur             INTEGER NOT NULL DEFAULT 0 CHECK (budget_eur >= 0),
  revenu_par_journee_eur INTEGER NOT NULL DEFAULT 500000 CHECK (revenu_par_journee_eur >= 0),

   
  avantage_domicile      INTEGER NOT NULL DEFAULT 5 CHECK (avantage_domicile BETWEEN 0 AND 30)
);


CREATE TABLE IF NOT EXISTS info_club(
  info_club_id         INTEGER PRIMARY KEY AUTOINCREMENT,
  club_id              INTEGER NOT NULL UNIQUE,
  nom_stade TEXT NOT NULL UNIQUE,
  stade_capacite INTEGER NOT NULL DEFAULT 20000 CHECK (stade_capacite >= 1000),
  url_logo TEXT NOT NULL UNIQUE,
  url_stade TEXT NOT NULL UNIQUE,

  nom_meilleur_buteur TEXT,
  FOREIGN KEY (club_id) REFERENCES clubs(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS saison_club (
  saison_id INTEGER NOT NULL,
  club_id   INTEGER NOT NULL,

  PRIMARY KEY (saison_id, club_id),
  FOREIGN KEY (saison_id) REFERENCES saisons(id) ON DELETE CASCADE,
  FOREIGN KEY (club_id)   REFERENCES clubs(id)   ON DELETE RESTRICT
);


CREATE TABLE IF NOT EXISTS etat_club_saison (
  club_id    INTEGER NOT NULL,
  saison_id  INTEGER NOT NULL,

  moral      INTEGER NOT NULL DEFAULT 50 CHECK (moral BETWEEN 0 AND 100),
  reputation INTEGER NOT NULL DEFAULT 50 CHECK (reputation BETWEEN 0 AND 100),

  PRIMARY KEY (club_id, saison_id),
  FOREIGN KEY (club_id)   REFERENCES clubs(id)   ON DELETE CASCADE,
  FOREIGN KEY (saison_id) REFERENCES saisons(id) ON DELETE CASCADE
);


-- =========================
-- 3) Joueurs & attributs (par saison)
-- =========================
CREATE TABLE IF NOT EXISTS joueurs (
  id                  INTEGER PRIMARY KEY AUTOINCREMENT,
  club_id             INTEGER,

  nom                 TEXT NOT NULL,
  age                 INTEGER NOT NULL CHECK (age BETWEEN 14 AND 60),
  numero              INTEGER NOT NULL CHECK (numero BETWEEN 1 AND 99),

  poste               TEXT NOT NULL CHECK (poste IN ('GARDIEN','DEFENSE','MILIEU','ATTAQUE')),
  pied                TEXT NOT NULL DEFAULT 'D' CHECK (pied IN ('D','G','A')),

  potentiel           INTEGER NOT NULL DEFAULT 50 CHECK (potentiel BETWEEN 0 AND 100),
  reputation          INTEGER NOT NULL DEFAULT 50 CHECK (reputation BETWEEN 0 AND 100),

  -- Economie joueur
  valeur_marche_eur   INTEGER NOT NULL DEFAULT 1000000 CHECK (valeur_marche_eur >= 0),
  salaire_semaine_eur INTEGER NOT NULL DEFAULT 20000 CHECK (salaire_semaine_eur >= 0),

  fin_contrat         TEXT,

  FOREIGN KEY (club_id) REFERENCES clubs(id) ON DELETE RESTRICT
);



CREATE TABLE IF NOT EXISTS attributs_joueur_saison (
  joueur_id     INTEGER NOT NULL,
  saison_id     INTEGER NOT NULL,

  vitesse     INTEGER NOT NULL CHECK (vitesse BETWEEN 1 AND 99),
  tir      INTEGER NOT NULL CHECK (tir BETWEEN 1 AND 99),
  passe         INTEGER NOT NULL CHECK (passe BETWEEN 1 AND 99),
  dribble      INTEGER NOT NULL CHECK (dribble BETWEEN 1 AND 99),
  defense    INTEGER NOT NULL CHECK (defense BETWEEN 1 AND 99),
  physique   INTEGER NOT NULL CHECK (physique BETWEEN 1 AND 99),
  sante      INTEGER NOT NULL DEFAULT 100 CHECK (sante BETWEEN 0 AND 100),

  moral         INTEGER NOT NULL DEFAULT 50 CHECK (moral BETWEEN 0 AND 100),

  nationalite TEXT NOT NULL,

  note_actuelle INTEGER NOT NULL DEFAULT 50 CHECK (note_actuelle BETWEEN 0 AND 100),

  PRIMARY KEY (joueur_id, saison_id),
  FOREIGN KEY (joueur_id) REFERENCES joueurs(id) ON DELETE CASCADE,
  FOREIGN KEY (saison_id) REFERENCES saisons(id) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS attributs_gardien_saison (
  joueur_id     INTEGER NOT NULL,
  saison_id     INTEGER NOT NULL,

  plongeon     INTEGER NOT NULL CHECK (plongeon BETWEEN 1 AND 99),
  jeu_a_la_main     INTEGER NOT NULL CHECK (jeu_a_la_main BETWEEN 1 AND 99),
  passe         INTEGER NOT NULL CHECK (passe BETWEEN 1 AND 99),
  reflexe   INTEGER NOT NULL CHECK (reflexe BETWEEN 1 AND 99),
  vitesse   INTEGER NOT NULL CHECK (vitesse BETWEEN 1 AND 99),
  position   INTEGER NOT NULL CHECK (position BETWEEN 1 AND 99),
  forme      INTEGER NOT NULL DEFAULT 100 CHECK (forme BETWEEN 0 AND 100),

  moral         INTEGER NOT NULL DEFAULT 50 CHECK (moral BETWEEN 0 AND 100),

  nationalite TEXT NOT NULL,

  note_actuelle INTEGER NOT NULL DEFAULT 50 CHECK (note_actuelle BETWEEN 0 AND 100),

  PRIMARY KEY (joueur_id, saison_id),
  FOREIGN KEY (joueur_id) REFERENCES joueurs(id) ON DELETE CASCADE,
  FOREIGN KEY (saison_id) REFERENCES saisons(id) ON DELETE CASCADE
);





-- =========================
-- 4) Matchs, résultats, compositions, événements
-- =========================
CREATE TABLE IF NOT EXISTS matchs (
  id                 INTEGER PRIMARY KEY AUTOINCREMENT,
  saison_id          INTEGER NOT NULL,

  journee            INTEGER NOT NULL CHECK (journee >= 1),

  club_domicile_id   INTEGER NOT NULL,
  club_exterieur_id  INTEGER NOT NULL,

  date_coup_envoi    TEXT,

  CHECK (club_domicile_id <> club_exterieur_id),

  FOREIGN KEY (saison_id)         REFERENCES saisons(id) ON DELETE CASCADE,
  FOREIGN KEY (club_domicile_id)  REFERENCES clubs(id)   ON DELETE RESTRICT,
  FOREIGN KEY (club_exterieur_id) REFERENCES clubs(id)   ON DELETE RESTRICT
);



CREATE TABLE IF NOT EXISTS resultats_matchs (
  match_id       INTEGER PRIMARY KEY,
  buts_domicile  INTEGER NOT NULL CHECK (buts_domicile BETWEEN 0 AND 30),
  buts_exterieur INTEGER NOT NULL CHECK (buts_exterieur BETWEEN 0 AND 30),
  simule_le      TEXT NOT NULL DEFAULT (datetime('now')),
  graine_alea    INTEGER,

  FOREIGN KEY (match_id) REFERENCES matchs(id) ON DELETE CASCADE
);




-- Composition sans remplacements
CREATE TABLE IF NOT EXISTS compositions_match (
  match_id       INTEGER NOT NULL,
  joueur_id      INTEGER NOT NULL,
  note_generale  INTEGER NOT NULL CHECK (note_generale BETWEEN 1 AND 100),
  finition INTEGER NOT NULL CHECK (finition BETWEEN 0 AND 100),
  collectif INTEGER NOT NULL CHECK (collectif BETWEEN 0 AND 100),
  forme_generale INTEGER NOT NULL CHECK (forme_generale BETWEEN 0 AND 100),


  est_titulaire  INTEGER NOT NULL DEFAULT 1 CHECK (est_titulaire IN (0,1)),
  poste_match    TEXT CHECK (poste_match IN ('GARDIEN','DEFENSE','MILIEU','ATTAQUE')),

  PRIMARY KEY (match_id, joueur_id),
  FOREIGN KEY (match_id)  REFERENCES matchs(id)  ON DELETE CASCADE,
  FOREIGN KEY (joueur_id) REFERENCES joueurs(id) ON DELETE RESTRICT
);



-- Événements : buts + cartons
CREATE TABLE IF NOT EXISTS evenement_matchs (
  id                    INTEGER PRIMARY KEY AUTOINCREMENT,
  match_id              INTEGER NOT NULL,
  minute                INTEGER NOT NULL CHECK (minute BETWEEN 0 AND 130),

  type_evenement        TEXT NOT NULL CHECK (type_evenement IN (
    'BUT','BUT_PEN','BUT_OG',
    'YELLOW','RED'
  )),

  joueur_id             INTEGER,
  joueur_secondaire_id  INTEGER,
  club_id               INTEGER,
  commentaire           TEXT,

  FOREIGN KEY (match_id)             REFERENCES matchs(id)  ON DELETE CASCADE,
  FOREIGN KEY (joueur_id)            REFERENCES joueurs(id) ON DELETE SET NULL,
  FOREIGN KEY (joueur_secondaire_id) REFERENCES joueurs(id) ON DELETE SET NULL,
  FOREIGN KEY (club_id)              REFERENCES clubs(id)   ON DELETE SET NULL
);



CREATE TABLE IF NOT EXISTS joueur_libre (
  id                    INTEGER PRIMARY KEY AUTOINCREMENT,
  joueur_id             INTEGER NOT NULL UNIQUE,
  
  
  FOREIGN KEY (joueur_id) REFERENCES joueurs(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transferts (
  id                           INTEGER PRIMARY KEY AUTOINCREMENT,
  saison_id                    INTEGER,

  joueur_id                    INTEGER NOT NULL,
  club_source_id               INTEGER,
  club_cible_id                INTEGER NOT NULL,

  coût_transfert               INTEGER NOT NULL CHECK (coût_transfert >= 0),

  date_transfert               TEXT NOT NULL DEFAULT (datetime('now')),

  FOREIGN KEY (saison_id)      REFERENCES saisons(id) ON DELETE SET NULL,
  FOREIGN KEY (joueur_id)      REFERENCES joueurs(id) ON DELETE CASCADE,
  FOREIGN KEY (club_source_id) REFERENCES clubs(id)   ON DELETE SET NULL,
  FOREIGN KEY (club_cible_id)  REFERENCES clubs(id)   ON DELETE RESTRICT
);



-- =========================
-- 6) Primes de fin de saison
-- =========================

CREATE TABLE IF NOT EXISTS primes_classement_saison (
  saison_id   INTEGER NOT NULL,
  rang        INTEGER NOT NULL CHECK (rang >= 1),
  montant_eur INTEGER NOT NULL CHECK (montant_eur >= 0),

  PRIMARY KEY (saison_id, rang),
  FOREIGN KEY (saison_id) REFERENCES saisons(id) ON DELETE CASCADE
);

-- =========================
-- 7) Index utiles
-- =========================

CREATE INDEX IF NOT EXISTS idx_saison_club_club
  ON saison_club(club_id);

CREATE INDEX IF NOT EXISTS idx_joueurs_club
  ON joueurs(club_id);

CREATE INDEX IF NOT EXISTS idx_attr_joueur_saison_saison
  ON attributs_joueur_saison(saison_id);

CREATE INDEX IF NOT EXISTS idx_matchs_saison_journee
  ON matchs(saison_id, journee);

CREATE INDEX IF NOT EXISTS idx_matchs_domicile
  ON matchs(club_domicile_id);

CREATE INDEX IF NOT EXISTS idx_matchs_exterieur
  ON matchs(club_exterieur_id);

CREATE INDEX IF NOT EXISTS idx_compositions_match_match
  ON compositions_match(match_id);

CREATE INDEX IF NOT EXISTS idx_evenement_matchs_match
  ON evenement_matchs(match_id);


CREATE INDEX IF NOT EXISTS idx_transferts_joueur
  ON transferts(joueur_id);