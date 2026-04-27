import { AddPlaylistForm } from "./components/AddPlaylistForm";
import { AppConfigForm } from "./components/AppConfigForm";
import { PlaylistList } from "./components/PlaylistList";
import type { AppConfig, PlaylistSummary } from "../../lib/types";

interface PlaylistHomeViewProps {
  playlists: PlaylistSummary[];
  busy: boolean;
  importBusy: boolean;
  configBusy: boolean;
  databasePath: string | null;
  appConfig: AppConfig;
  configPath: string | null;
  configSource: string | null;
  error: string | null;
  successMessage: string | null;
  onImport: (url: string) => Promise<void>;
  onSaveConfig: (config: AppConfig) => Promise<void>;
  onOpenPlaylist: (playlistId: string) => void;
}

export function PlaylistHomeView({
  playlists,
  busy,
  importBusy,
  configBusy,
  databasePath,
  appConfig,
  configPath,
  configSource,
  error,
  successMessage,
  onImport,
  onSaveConfig,
  onOpenPlaylist,
}: PlaylistHomeViewProps) {
  return (
    <main className="app-shell">
      <section className="hero">
        <div className="stack gap-sm">
          <p className="eyebrow">playlist-browser</p>
          <h1>Bibliothèque locale de playlists YouTube</h1>
          <p className="hero-copy">
            Importez des métadonnées de playlist en local, retrouvez vos listes
            rapidement et lancez la lecture dans une interface simple.
          </p>
        </div>

        {databasePath ? (
          <p className="muted small">
            Base locale : <code>{databasePath}</code>
          </p>
        ) : null}
      </section>

      <AddPlaylistForm busy={importBusy} onSubmit={onImport} />
      <AppConfigForm
        busy={configBusy}
        config={appConfig}
        configPath={configPath}
        configSource={configSource}
        onSubmit={onSaveConfig}
      />

      {error ? <div className="banner banner-error">{error}</div> : null}
      {successMessage ? (
        <div className="banner banner-success">{successMessage}</div>
      ) : null}

      {busy ? (
        <section className="panel">
          <p>Chargement des playlists...</p>
        </section>
      ) : (
        <PlaylistList playlists={playlists} onOpenPlaylist={onOpenPlaylist} />
      )}
    </main>
  );
}
