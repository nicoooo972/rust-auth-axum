# Rust Auth with Axum

Ce projet est une implémentation d'un système d'authentification en **Rust** utilisant le framework **Axum**. Il met en œuvre des pratiques modernes pour la création d'une API sécurisée et performante.

---

## 🚀 Fonctionnalités

- 🔐 Authentification utilisateur (enregistrement et connexion)
- 📄 Gestion des tokens JWT pour la sécurisation des endpoints
- 🛡️ Middleware pour vérifier les permissions et l'accès
- 💾 Intégration avec une base de données (PostgreSQL via Docker)
- 📦 Architecture modulaire et maintenable

---

## 🛠️ Prérequis

Avant de lancer le projet, assurez-vous d'avoir :

- [Rust](https://www.rust-lang.org/) installé (version stable recommandée)
- [Cargo](https://doc.rust-lang.org/cargo/) pour gérer les dépendances
- [Docker](https://www.docker.com/) pour la base de données

---

## ⚙️ Installation

1. Clonez le dépôt :
   ```bash
   git clone https://github.com/nicoooo972/rust-auth-axum.git
   cd rust-auth-axum
   ```

2. Installez les dépendances :
   ```bash
   cargo build
   ```

3. Configurez les variables d'environnement (voir la section Configuration)

4. Lancez la base de données :
   ```bash
   docker-compose up -d
   ```

5. Lancez le serveur :
   ```bash
   cargo run
   ```

Le serveur sera accessible à l'adresse http://localhost:3000.

## 📝 Configuration Docker

Le projet utilise Docker pour gérer la base de données PostgreSQL et pgAdmin. Voici la configuration dans le fichier `docker-compose.yml` :

```yaml
services:
  postgres:
    image: postgres:latest
    container_name: postgres
    ports:
      - '6500:5432'
    volumes:
      - progresDB:/var/lib/postgresql/data
    env_file:
      - ./.env
  pgAdmin:
    image: dpage/pgadmin4
    container_name: pgAdmin
    env_file:
      - ./.env
    ports:
      - '5050:80'
volumes:
  progresDB:
```

Créez un fichier `.env` à la racine du projet avec les variables suivantes :

```env
# Configuration PostgreSQL
POSTGRES_USER=votre_utilisateur
POSTGRES_PASSWORD=votre_mot_de_passe
POSTGRES_DB=nom_de_votre_base
DATABASE_URL=postgres://votre_utilisateur:votre_mot_de_passe@localhost:6500/nom_de_votre_base

# Configuration pgAdmin
PGADMIN_DEFAULT_EMAIL=votre_email
PGADMIN_DEFAULT_PASSWORD=votre_mot_de_passe

# Configuration JWT
JWT_SECRET=un_secret_très_sécurisé
```

Une fois lancé :
- PostgreSQL sera accessible sur le port 6500
- pgAdmin sera accessible sur http://localhost:5050

## 📂 Structure du projet

Voici un aperçu de l'organisation du code :

```
src/
├── handlers/       # Gestionnaires des endpoints
├── middlewares/    # Middleware (authentification, etc.)
├── models/         # Structures de données
├── routes/         # Définition des routes
├── services/       # Logique métier
└── main.rs         # Point d'entrée du projet
```

## 🚧 Roadmap

* Implémenter la gestion des rôles utilisateurs
* Ajouter la documentation OpenAPI
* Intégrer des tests unitaires et d'intégration
* Ajouter des fonctionnalités avancées (authentification sociale, etc.)

## 🤝 Contribuer

Les contributions sont les bienvenues ! Si vous avez des suggestions ou souhaitez signaler des bugs, ouvrez une issue ou soumettez une pull request.

## 📄 Licence

Ce projet est sous licence MIT.

## ❤️ Remerciements
Merci à wpcodevo, clone de base : https://github.com/wpcodevo/jwt-auth-axum-rust, des modifications, ont été faites et vont être faites afin de correspondre à mon projet : https://github.com/nicoooo972/sowesport---front/
Merci à la communauté Rust pour ses contributions et son soutien !
