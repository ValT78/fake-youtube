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
- `yt-dlp` installé pour résoudre le flux vidéo
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

Variable optionnelle pour forcer le binaire `yt-dlp` :

```bash
export YTDLP_PATH="/chemin/vers/yt-dlp"
```

Utilise `YTDLP_PATH` si `yt-dlp` n'est pas présent dans le `PATH` du shell qui lance Tauri.

## Lancement en développement

Pour lancer l'application desktop complète :

```bash
npm run dev
```

Sous Linux / WSL, tu peux vérifier la présence de VLC avec :

```bash
which vlc
```

Et vérifier `yt-dlp` avec :

```bash
which yt-dlp
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
- **Lecture externe** : `yt-dlp` résout une URL de flux, puis VLC lit cette URL
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
- lecture déléguée à VLC via une URL de flux résolue par `yt-dlp`
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
- résolution de flux vidéo par `yt-dlp` avant lancement de VLC

## Build Windows

Oui, l'application peut fonctionner sous Windows en natif si :

- Tauri est buildé sur une machine Windows
- VLC est installé
- `yt-dlp` est installé
- les binaires sont visibles dans le `PATH` ou configurés via `VLC_PATH` et `YTDLP_PATH`

Le code Rust cherche déjà :

- `vlc.exe` ou `C:\Program Files\VideoLAN\VLC\vlc.exe`
- `yt-dlp.exe` ou `yt-dlp` dans le `PATH`

L'objectif est justement de tester si la lecture est plus fluide en natif Windows qu'avec WSLg.

### Pourquoi tester en Windows natif

Sous Windows, Tauri utilise **WebView2**. La doc officielle précise que Tauri s'appuie sur WebView2 côté Windows, basé sur Edge/Chromium, alors que Linux repose sur `webkit2gtk`.

Sources officielles :

- Prérequis Tauri : https://v2.tauri.app/start/prerequisites/
- Versions de webview : https://v2.tauri.app/reference/webview-versions/

En pratique, cela retire la surcouche `WSL2 + WSLg + WebKitGTK`, qui est le suspect principal de tes problèmes de fluidité.

### Préparer Windows

Installe ces dépendances sur Windows :

1. Node.js
2. Rust avec la toolchain `stable-msvc`
3. Microsoft C++ Build Tools avec `Desktop development with C++`
4. WebView2 Runtime si nécessaire
5. VLC
6. `yt-dlp`

Pour `yt-dlp`, l'installation officielle Windows peut se faire par :

```powershell
winget install yt-dlp
```

ou en téléchargeant `yt-dlp.exe`.

Référence officielle `yt-dlp` :

- https://github.com/yt-dlp/yt-dlp/wiki/Installation
- https://github.com/yt-dlp/yt-dlp

### Vérifier l'environnement Windows

Dans PowerShell :

```powershell
node -v
npm -v
rustup default stable-msvc
where.exe vlc
where.exe yt-dlp
```

Si `vlc` ou `yt-dlp` ne sont pas trouvés, tu peux forcer leurs chemins :

```powershell
$env:VLC_PATH="C:\Program Files\VideoLAN\VLC\vlc.exe"
$env:YTDLP_PATH="C:\chemin\vers\yt-dlp.exe"
```

Tu peux aussi définir ces variables au niveau utilisateur dans Windows si tu veux lancer l'app packagée sans ouvrir PowerShell au préalable.

### Lancer en développement sur Windows

Depuis une session PowerShell ouverte dans le dépôt :

```powershell
npm install
$env:YOUTUBE_API_KEY="votre_cle"
npm run dev
```

Puis teste :

1. import d'une playlist
2. ouverture d'une playlist
3. clic sur `Lire dans VLC`

Le comportement attendu est :

1. l'app appelle `yt-dlp`
2. `yt-dlp` résout une URL de flux directe
3. VLC s'ouvre avec cette URL

### Exporter une build Windows

Toujours depuis Windows natif :

```powershell
npm install
$env:YOUTUBE_API_KEY="votre_cle"
npm run tauri:build
```

Les bundles Windows seront générés dans :

```text
src-tauri\target\release\bundle\
```

Selon la configuration Tauri/outillage disponible, tu obtiendras typiquement un exécutable installé dans un bundle `nsis` et potentiellement un `msi`.

### Tester une build packagée

Pour vérifier qu'une build installée retrouve bien VLC et `yt-dlp`, je te conseille d'abord ce chemin simple :

1. installer VLC normalement
2. installer `yt-dlp` via `winget`
3. ouvrir une nouvelle session Windows
4. installer puis lancer l'application packagée

Si la build ne trouve pas l'un des deux outils, le plus simple pour le diagnostic est de relancer l'application depuis un terminal PowerShell avec :

```powershell
$env:VLC_PATH="C:\Program Files\VideoLAN\VLC\vlc.exe"
$env:YTDLP_PATH="C:\chemin\vers\yt-dlp.exe"
```

### Limite actuelle de cette approche

Même sous Windows natif, la vidéo ne sera pas lue *dans* l'application :

- l'app reste la bibliothèque locale
- `yt-dlp` résout le flux
- VLC fait la lecture

C'est volontairement la solution la plus simple et la plus robuste pour tester la fluidité en environnement natif.
