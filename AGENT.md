# AGENT.md

## Mission produit

Construire une application desktop locale nommée provisoirement `playlist-browser` avec :

- stack : **Tauri 2 + TypeScript + Rust + SQLite**
- objectif : permettre à un utilisateur de **saisir l’URL de n’importe quelle playlist YouTube** et de **consulter localement ses métadonnées**
- UX cible : extrêmement simple, sans jargon, sans setup technique visible
- usage : application personnelle / d’appoint, non vendue, non multi-tenant, sans backend cloud

L’application ne doit pas reproduire YouTube. Elle doit fournir une **bibliothèque locale minimaliste**.

---

## Vision UX

### Parcours principal

1. L’utilisateur ouvre l’application.
2. Il voit l’écran d’accueil avec la liste des playlists déjà importées.
3. Il clique sur “Ajouter une playlist”.
4. Il colle une URL YouTube de playlist.
5. L’application :
   - extrait le `playlistId`
   - récupère les métadonnées de la playlist et de ses vidéos via l’API YouTube
   - stocke ces métadonnées localement en SQLite
6. La playlist apparaît dans l’accueil.
7. En cliquant sur une playlist :
   - la première vidéo démarre
   - les autres vidéos apparaissent dans une colonne latérale
   - l’utilisateur peut cliquer sur une vidéo, ou aller à la suivante / précédente

### Exigences UX

- l’app doit être compréhensible sans documentation
- une seule responsabilité principale : **consulter des playlists**
- éviter toute fonctionnalité annexe non indispensable
- afficher des messages d’erreur clairs et non techniques
- démarrage rapide grâce au stockage local
- pas de dépendance à un backend distant

---

## Décisions d’architecture

### Architecture générale

Application desktop locale monolithique :

- **Frontend** : TypeScript, framework léger autorisé
- **Shell desktop / système** : Tauri 2
- **Backend local** : commandes Rust Tauri
- **Base locale** : SQLite
- **Aucun backend HTTP distant**
- **Aucune authentification locale complexe**
- **Aucun compte applicatif**

### Pourquoi

- simplicité de packaging
- simplicité d’usage
- faible maintenance
- faible surface d’erreur
- bonne réactivité grâce au cache local

---

## Contraintes techniques structurantes

### 1. Données stockées

Stocker uniquement les **métadonnées** utiles :

- playlists
- vidéos
- appartenance vidéo -> playlist
- position dans la playlist
- miniature URL
- durée si disponible
- chaîne
- date de publication si disponible
- date d’import / date de dernière synchro

Ne **pas** stocker les vidéos en local en v1.

### 2. Lecture vidéo

#### V1
Lecture via **URL YouTube / player embarqué simple**, avec priorité à la simplicité d’intégration.

#### V2 éventuelle
Lecteur embarqué plus sophistiqué avec fallback si l’embed échoue.

### 3. Synchronisation

Importer au minimum :

- métadonnées de playlist
- items de playlist paginés
- métadonnées de vidéo nécessaires à l’affichage

Pas de synchronisation en continu en v1.

### 4. Source des playlists

En v1, une playlist est importée via :

- URL de playlist YouTube
- ou identifiant de playlist déjà extrait si utile en interne

Ne pas dépendre des playlists “mine=true” pour le cœur du produit.
Le cœur du produit est : **import d’une playlist par URL**.

---

## Principes d’implémentation

### Simplicité d’abord

Toujours choisir :

- moins d’écrans
- moins de paramètres
- moins de magie
- moins de couches
- moins d’abstractions prématurées

### Séparation claire des responsabilités

- frontend : présentation, navigation, état UI
- rust : accès système, appels externes sensibles, parsing central, commandes
- sqlite : persistance locale
- code youtube : centralisé dans un module dédié

### Pas de dette “invisible”

Éviter :

- gros services globaux opaques
- logique métier répartie dans des composants UI
- SQL dispersé partout
- types implicites ou `any`
- duplication de schémas TS / Rust sans stratégie claire

---

## Structure de projet souhaitée

### Frontend

- `src/app/`
  - shell application
  - routing simple
- `src/features/playlists/`
  - liste accueil
  - détail playlist
  - formulaire ajout playlist
- `src/features/player/`
  - lecteur
  - file de lecture latérale
- `src/lib/`
  - api tauri
  - types frontend
  - helpers UI

### Rust / Tauri

- `src-tauri/src/main.rs`
- `src-tauri/src/commands/`
  - `playlist_commands.rs`
  - `db_commands.rs`
- `src-tauri/src/services/`
  - `youtube.rs`
  - `playlist_parser.rs`
  - `sync.rs`
- `src-tauri/src/db/`
  - `mod.rs`
  - `migrations/`
- `src-tauri/src/models/`
  - playlist/video records
- `src-tauri/src/errors.rs`

---

## Base de données

Utiliser SQLite avec migrations.

### Tables minimales

#### playlists
- `id` TEXT PRIMARY KEY
- `youtube_playlist_id` TEXT NOT NULL UNIQUE
- `title` TEXT NOT NULL
- `description` TEXT
- `channel_title` TEXT
- `thumbnail_url` TEXT
- `video_count` INTEGER
- `source_url` TEXT NOT NULL
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL
- `last_synced_at` TEXT

#### videos
- `id` TEXT PRIMARY KEY
- `youtube_video_id` TEXT NOT NULL UNIQUE
- `title` TEXT NOT NULL
- `description` TEXT
- `channel_title` TEXT
- `thumbnail_url` TEXT
- `published_at` TEXT
- `duration_iso8601` TEXT
- `duration_seconds` INTEGER
- `created_at` TEXT NOT NULL
- `updated_at` TEXT NOT NULL

#### playlist_items
- `id` TEXT PRIMARY KEY
- `playlist_id` TEXT NOT NULL
- `video_id` TEXT NOT NULL
- `youtube_playlist_item_id` TEXT
- `position` INTEGER NOT NULL
- `created_at` TEXT NOT NULL
- UNIQUE(`playlist_id`, `position`)
- UNIQUE(`playlist_id`, `video_id`)

Prévoir les index utiles sur :
- `youtube_playlist_id`
- `youtube_video_id`
- `playlist_id, position`

---

## API YouTube

Le code doit être conçu pour fonctionner avec l’API YouTube Data v3.

### Règles

- centraliser tous les appels YouTube dans `youtube.rs`
- ne jamais appeler YouTube directement depuis l’UI
- gérer la pagination des playlist items
- parser proprement les réponses
- retourner des erreurs métier propres

### Cas à couvrir

- URL invalide
- URL valide mais sans playlist ID
- playlist inexistante
- playlist privée / inaccessible
- quota dépassé
- vidéo supprimée / indisponible
- miniature absente
- champs optionnels manquants

### Parsing URL

Supporter les formes usuelles :
- `https://www.youtube.com/playlist?list=...`
- `https://youtube.com/playlist?list=...`
- `https://www.youtube.com/watch?v=xxx&list=...`
- `https://youtu.be/...?...&list=...`

Le parsing doit être robuste, testé, et centralisé dans `playlist_parser.rs`.

---

## Interface utilisateur

### Écran d’accueil

Contient :
- titre d’application
- bouton “Ajouter une playlist”
- liste des playlists importées

Chaque carte playlist affiche au minimum :
- miniature
- titre
- chaîne
- nombre de vidéos
- date de dernière synchro

### Écran playlist

Contient :
- lecteur vidéo principal
- titre de playlist
- liste latérale des vidéos
- bouton synchroniser
- actions suivante / précédente

### Navigation

Simple :
- Accueil
- Playlist détail

Pas de routing complexe.

---

## Choix UI

Privilégier :
- composants sobres
- accessibilité clavier minimale
- indicateurs de chargement simples
- états vides bien traités

Éviter :
- thèmes compliqués
- animations inutiles
- préférences avancées
- surcharge visuelle

---

## Qualité de code attendue

### TypeScript

- mode strict
- aucun `any` non justifié
- types métier explicites
- séparation DTO / modèles UI si nécessaire

### Rust

- erreurs typées
- modules petits et cohérents
- pas de `unwrap()` dans les chemins de production
- fonctions courtes
- logs utiles

### SQL

- migrations versionnées
- requêtes regroupées
- pas de SQL inline dispersé si cela nuit à la lisibilité

---

## Tests attendus

### À minima

#### Rust
- parsing d’URL de playlist
- mapping des réponses YouTube essentielles
- logique de synchro pure si isolable

#### Frontend
- rendu de la liste de playlists
- rendu de la file de vidéos
- navigation suivante / précédente

### Critères de priorité test
1. parsing URL
2. persistance SQLite
3. navigation playlist / vidéo
4. états d’erreur

---

## Dépendances

Préférer peu de dépendances.

### Autorisées si elles simplifient vraiment
- framework frontend courant
- librairie légère de state management si besoin réel
- tauri plugin SQL
- utilitaires de validation légers

### À éviter
- state machine lourde
- ORM inutilement complexe
- framework CSS surdimensionné
- dépendances abandonnées ou non officielles sans raison

---

## Roadmap v1

### Phase 1
- initialisation Tauri 2 + TypeScript
- fenêtre desktop fonctionnelle
- structure de dossiers
- plugin SQLite
- migrations initiales

### Phase 2
- parsing URL playlist
- formulaire “Ajouter une playlist”
- service Rust d’import de playlist

### Phase 3
- stockage playlist / vidéos / items
- écran d’accueil avec playlists importées

### Phase 4
- écran playlist
- lecteur vidéo
- colonne des vidéos
- navigation précédente / suivante

### Phase 5
- gestion d’erreurs propre
- tests essentiels
- README de développement

---

## Hors périmètre v1

Ne pas implémenter maintenant :

- comptes multi-utilisateurs
- cloud sync
- téléchargement vidéo local
- mode offline vidéo
- recommandations
- tags personnels
- édition de playlists YouTube
- import massif de tout un compte
- architecture plugin
- analytics
- télémétrie

---

## Critères de réussite

Le projet est réussi si :

1. un développeur peut cloner et lancer l’app facilement
2. un utilisateur peut coller une URL de playlist et l’importer
3. la playlist apparaît ensuite sur l’accueil
4. cliquer sur la playlist ouvre la lecture de la première vidéo
5. les vidéos suivantes/précédentes sont navigables simplement
6. les métadonnées sont persistées localement proprement
7. le code est lisible, modulaire et extensible

---

## Consignes pour l’agent

Quand tu modifies le projet :

- commence par produire un squelette minimal exécutable
- fais des commits/logiques incrémentales même si tu ne commit pas réellement
- garde les décisions simples
- documente les arbitrages importants dans le README
- ne rajoute pas de fonctionnalités non demandées
- en cas d’incertitude, choisir l’option la plus simple et la plus maintenable