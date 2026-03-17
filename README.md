Remplissage de la db avec les csv

rm -f db/simulation.db
sqlite3 db/simulation.db < db/schema.sql

sqlite3 db/simulation.db <<EOF
.mode csv
.import --skip 1 db/data/competitions.csv competitions
.import --skip 1 db/data/saisons.csv saisons
.import --skip 1 db/data/clubs.csv clubs
.import --skip 1 db/data/saison_club.csv saison_club
.import --skip 1 db/data/etat_club_saison.csv etat_club_saison
.import --skip 1 db/data/primes_classement_saison.csv primes_classement_saison
.import --skip 1 db/data/stats_joueurs.csv attributs_joueur_saison
.import --skip 1 db/data/stats_gardiens.csv attributs_gardien_saison
EOF