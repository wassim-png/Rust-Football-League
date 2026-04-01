# Rust Football League

Un jeu de gestion de football simplifié inspiré de Football Manager, développé en **Rust** avec **SQLite** et **egui**.

## Présentation

Rust Football League permet de simuler la gestion d'un championnat de football : choisir son club, composer son équipe, acheter et vendre des joueurs au mercato, simuler des matchs et suivre le classement au fil des journées.

Ce projet rust est développé par **Wassim Ben Nacef**, **Yoan Bastides**, **Matteo Denee** et **Ismael Ben Ayed**.

## Fonctionnalités

- **Sélection de club** — Choisir parmi 18 clubs avec budget, réputation et stade
- **Composition d'équipe** — Sélectionner 11 joueurs avec la formation de notre choix, triés par note générale
- **Simulation de matchs** — Moteur probabiliste basé sur la note de la compo, le collectif et la forme
- **Mercato** — Acheter des joueurs libres ou racheter des contrats, vendre ses joueurs, recevoir des offres IA générées aléatoirement
- **Calendrier** — 34 journées générées automatiquement par algorithme round-robin (18 clubs, 9 matchs par journée)
- **Classement** — 3 points par victoire, 1 point par match nul et 0 point par défaite


## Technologies

| Technologie | Rôle |
|---|---|
| **Rust** | Logique du jeu, simulation, règles métier |
| **SQLite** | Base de données (16 tables, via rusqlite) |
| **egui / eframe** | Interface graphique native |
| **rand** | Génération aléatoire (simulation, offres IA) |
| **serde** | Sérialisation des données |
| **csv** | Import des données initiales |

## Architecture

Le projet suit une architecture en couches avec les design patterns **Facade**, **DAO** et **Command** :

```
Interface egui (10 écrans)
        ↓
Facades (8 modules — point d'entrée unique par domaine)
        ↓
Managers (logique métier et calculs)
        ↓
DAO — Traits + implémentations SQL (rusqlite)
        ↓
SQLite (simulation.db — 16 tables)
```

Toutes les connexions passent par un `Arc<Connection>` partagé. Les mutations de la base sont différées après le rendu egui (pattern Command) pour éviter les conflits d'emprunt.

### Structure des fichiers (63 fichiers .rs)

```
src/
├── main.rs                    Point d'entrée
├── app.rs                     État global + routeur d'écrans
├── models.rs                  Structures de données
├── database.rs                Init schéma SQLite
├── page/                      Accueil + Dashboard
├── selection_club/            Sélection d'équipe
├── infos_club/                Fiche club (stade, stats)
├── composition/               Choix des 11 joueurs
├── simulation/                Moteur de simulation
├── mercato/                   Transferts (achat/vente)
├── calendrier/                Planning des journées
└── prochain_match/            Prochain match à jouer
```

Chaque module suit la même structure : `ui/` → `business_logic/` (Facade + Manager) → `persist/` (DAO trait + SQL impl).

## Base de données

16 tables organisées en 5 groupes :

- **Compétition** — competitions, saisons
- **Clubs** — clubs, info_club, saison_club, etat_club_saison
- **Joueurs** — joueurs, joueurs_libres, attributs_joueur_saison, attributs_gardien_saison
- **Matchs** — matchs, resultats_matchs, compositions_match, evenement_matchs
- **Économie** — transferts, primes_classement_saison

## Installation et lancement

```bash
# Cloner le projet
git clone <git@github.com:wassim-png/Rust-Football-League.git>
cd rust-football-league

# Compiler et lancer
cargo run
```
