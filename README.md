# playlist-browser

Application desktop locale pour importer et consulter des playlists YouTube avec **Tauri 2**, **React + TypeScript**, **Rust** et **SQLite**.

L'application reste volontairement simple :

- import d'une playlist via son URL YouTube
- stockage local des métadonnées dans SQLite
- accueil avec bibliothèque locale
- écran détail avec ouverture dans VLC et navigation précédente / suivante

## Prérequis

### Tous environnements

- Node.js 22+
- npm 10+
- Rust stable 1.77.2+ via `rustup`
- une clé **YouTube Data API v3**
- VLC installé si tu veux tester la lecture externe

### Linux (Tauri)

Les bibliothèques système suivantes sont nécessaires pour compiler Tauri sous Debian/Ubuntu :

```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  libxdo-dev \
  librsvg2-dev \
  libsqlite3-dev
```

Note : dans cet environnement de travail, cette étape n'a pas pu être exécutée automatiquement car `sudo` demande un mot de passe interactif.

## Installation

```bash
npm install
```

Si Rust n'est pas encore installé :

```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
```

## Variables d'environnement

Créer un shell avec la variable suivante avant de lancer Tauri :

```bash
export YOUTUBE_API_KEY="votre_cle_youtube_data_api_v3"
```

L'import réel d'une playlist dépend de cette clé. Sans elle, l'application renvoie un message d'erreur explicite.

Variable optionnelle pour forcer le binaire VLC :

```bash
export VLC_PATH="/chemin/vers/vlc"
```

Utilise `VLC_PATH` si `vlc` n'est pas présent dans le `PATH` du shell qui lance Tauri.

## Lancement en développement

Pour lancer l'application desktop complète :

```bash
npm run dev
```

Sous Linux / WSL, tu peux vérifier la présence de VLC avec :

```bash
which vlc
```

Pour vérifier uniquement le frontend :

```bash
npm run web:build
npm test
```

## Structure du projet

```text
.
├── src/
│   ├── app/
│   ├── features/
│   │   ├── player/
│   │   └── playlists/
│   └── lib/
├── src-tauri/
│   ├── capabilities/
│   ├── src/
│   │   ├── commands/
│   │   ├── db/
│   │   ├── models/
│   │   └── services/
│   └── tauri.conf.json
└── AGENT.md
```

## Base locale SQLite

Base gérée via **Tauri SQL plugin** et migrations au démarrage.

Tables initiales :

- `playlists`
- `videos`
- `playlist_items`

Le fichier SQLite est stocké dans le répertoire de configuration de l'application Tauri.

## Architecture retenue

- **Frontend React** : deux vues effectives, accueil et détail playlist
- **État UI** : hooks React simples, sans state manager externe
- **Rust** : parsing d'URL, accès YouTube, logique de synchronisation, commandes Tauri
- **SQLite** : persistance locale avec requêtes explicites, sans ORM

## Commandes Tauri disponibles

- `parse_playlist_url`
- `import_playlist`
- `list_playlists`
- `get_playlist_detail`
- `open_video_in_vlc`
- `database_status`

## Tests

Frontend validés :

- rendu de la liste de playlists
- navigation précédente / suivante du lecteur

Rust prévus dans le dépôt :

- tests de parsing d'URL
- tests de mapping YouTube
- tests de logique pure de synchro

Note : l'exécution `cargo test` est actuellement bloquée sur cette machine tant que les dépendances système GTK/WebKit de Tauri ne sont pas installées.

## Limites actuelles de la v1

- pas de téléchargement vidéo local
- pas de synchronisation automatique en arrière-plan
- pas de comptes multi-utilisateurs
- pas de backend distant
- lecture déléguée à VLC via l'URL YouTube de la vidéo
- l'import dépend d'une clé YouTube Data API v3 valide

## État du projet

### Implémenté

- squelette Tauri 2 + React + TypeScript
- TypeScript strict
- configuration Tauri + capability SQL
- migration SQLite initiale
- parsing d'URL YouTube centralisé côté Rust
- service YouTube structuré dans `youtube.rs`
- logique de synchronisation dans `sync.rs`
- accueil avec import et liste locale
- écran détail playlist avec ouverture dans VLC et file latérale
- navigation précédente / suivante
- gestion d'erreurs utilisateur de base

## Build Windows

Pour tester le comportement natif Windows, fais le build depuis une machine Windows plutôt que depuis WSL.

Prérequis Windows Tauri :

- Microsoft C++ Build Tools avec `Desktop development with C++`
- Microsoft Edge WebView2
- Rust stable

Références officielles :

- https://v2.tauri.app/start/prerequisites/
- https://v2.tauri.app/reference/webview-versions/

Étapes sur Windows :

```powershell
npm install
$env:YOUTUBE_API_KEY="votre_cle"
npm run tauri:build
```

Si VLC n'est pas détecté automatiquement :

```powershell
$env:VLC_PATH="C:\Program Files\VideoLAN\VLC\vlc.exe"
```

Les bundles Windows seront générés dans `src-tauri\target\release\bundle\`.
