# Greenhouse Controller

Greenhouse Controller est un système de contrôle automatisé pour les serres, développé en utilisant Rust et Raspberry Pi. Il permet de surveiller les conditions environnementales à l'intérieur d'une serre, de contrôler automatiquement les équipements de la serre en fonction de paramètres prédéfinis et de fournir une interface utilisateur conviviale pour visualiser les données et configurer les paramètres de contrôle.

## Fonctionnalités principales

- Surveillance en temps réel des conditions environnementales (température, humidité, luminosité) à l'aide de capteurs.
- Contrôle automatique des équipements de la serre (ventilation, chauffage, éclairage) en fonction des paramètres de contrôle définis.
- Interface utilisateur web intuitive pour visualiser les données des capteurs et configurer les paramètres de contrôle.
- Stockage des données des capteurs et des paramètres de contrôle dans une base de données SQLite.
- API REST pour interagir avec le système et récupérer les données des capteurs et les paramètres de contrôle.

## Prérequis

- Raspberry Pi avec Rust installé
- Capteurs de température, d'humidité et de luminosité compatibles
- Relais pour contrôler les équipements de la serre (ventilation, chauffage, éclairage)

## Installation

1. Clonez ce dépôt sur votre Raspberry Pi : git clone https://github.com/LukaChassaing/greenhouse-controller.git
2. Accédez au répertoire du projet : cd greenhouse-controller
3. Compilez le projet : cargo build --release
4. Configurez les variables d'environnement dans un fichier `.env` à la racine du projet :
DATABASE_URL=sqlite:database.db
API_PORT=8080
5. Exécutez l'application : cargo run --release

## Configuration des capteurs et des relais

Avant d'utiliser le système, vous devez configurer les capteurs et les relais connectés à votre Raspberry Pi. Suivez les étapes ci-dessous :

1. Connectez les capteurs de température, d'humidité et de luminosité aux broches appropriées du Raspberry Pi.
2. Connectez les relais pour contrôler les équipements de la serre (ventilation, chauffage, éclairage) aux broches appropriées du Raspberry Pi.
3. Mettez à jour les constantes dans le fichier `sensors.rs` pour spécifier les broches utilisées pour chaque capteur.
4. Mettez à jour les constantes dans le fichier `control.rs` pour spécifier les broches utilisées pour chaque relais.

## Utilisation

Une fois le système installé et configuré, vous pouvez y accéder via l'interface utilisateur web à l'adresse `http://adresse-ip-raspberry-pi:8080`.

L'interface utilisateur vous permet de :
- Visualiser les données en temps réel des capteurs (température, humidité, luminosité).
- Configurer les paramètres de contrôle pour la température, l'humidité et la luminosité.
- Consulter l'historique des données des capteurs.

Vous pouvez également interagir avec le système via l'API REST. Les endpoints disponibles sont :
- `GET /measurements` : Récupère les dernières mesures des capteurs.
- `GET /control_settings` : Récupère les paramètres de contrôle actuels.
- `PUT /control_settings` : Met à jour les paramètres de contrôle.

## Structure du projet

- `src/main.rs` : Point d'entrée du programme, orchestre l'exécution des différents modules.
- `src/sensors.rs` : Gère la lecture des données des capteurs.
- `src/control.rs` : Gère le contrôle des équipements de la serre via les relais.
- `src/database.rs` : Gère les interactions avec la base de données SQLite.
- `src/api.rs` : Implémente l'API REST pour interagir avec le système.
- `static/` : Contient les fichiers statiques pour l'interface utilisateur web.
- `Cargo.toml` : Fichier de configuration de Rust et des dépendances du projet.
- `.env` : Fichier contenant les variables d'environnement (à créer lors de l'installation).

## Contribution

Les contributions à ce projet sont les bienvenues ! Si vous souhaitez apporter des améliorations, corriger des bogues ou ajouter de nouvelles fonctionnalités, veuillez suivre les étapes ci-dessous :

1. Fork ce dépôt.
2. Créez une nouvelle branche pour vos modifications : `git checkout -b ma-branche`.
3. Effectuez les modifications souhaitées et committez-les : `git commit -m 'Description des modifications'`.
4. Poussez vos modifications vers votre fork : `git push origin ma-branche`.
5. Ouvrez une pull request sur ce dépôt en décrivant vos modifications.

## Licence

Ce projet est sous licence MIT. Veuillez consulter le fichier [LICENSE](LICENSE) pour plus d'informations.
