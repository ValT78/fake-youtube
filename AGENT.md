# AGENT.md

## Mission produit

Construire une application desktop locale nommée provisoirement `playlist-browser` avec :

- stack : **Tauri 2 + TypeScript + Rust + SQLite**
- objectif : permettre à un utilisateur de **saisir l’URL de n’importe quelle playlist YouTube** et de **consulter localement ses métadonnées**
- UX cible : extrêmement simple, sans jargon, sans setup technique visible
- usage : application personnelle / d’appoint, non vendue, non multi-tenant, sans backend cloud

L’application ne doit pas reproduire YouTube. Elle doit fournir une **bibliothèque locale minimaliste**.

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

## Consignes pour l’agent

Quand tu modifies le projet :

- commence par produire un squelette minimal exécutable
- fais des commits/logiques incrémentales même si tu ne commit pas réellement
- garde les décisions simples
- documente les arbitrages importants dans le README
- ne rajoute pas de fonctionnalités non demandées
- en cas d’incertitude, choisir l’option la plus simple et la plus maintenable