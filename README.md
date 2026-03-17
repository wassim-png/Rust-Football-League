Remplissage de la db avec les csv

sqlite3 db/simulation.db <<EOF
.mode csv
.import db/data/competitions.csv competitions
.import db/data/saisons.csv saisons
.import db/data/clubs.csv clubs
.import db/data/saison_club.csv saison_club
.import db/data/etat_club_saison.csv etat_club_saison
.import db/data/info_club.csv info_club
.import db/data/primes_classement_saison.csv primes_classement_saison
EOF