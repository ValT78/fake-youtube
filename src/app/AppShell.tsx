import { useEffect, useState } from "react";
import { PlaylistDetailView } from "../features/playlists/PlaylistDetailView";
import { PlaylistHomeView } from "../features/playlists/PlaylistHomeView";
import { ensureLocalDatabase } from "../lib/db";
import { toErrorMessage } from "../lib/errors";
import {
  getAppConfig,
  getDatabaseStatus,
  importPlaylist,
  listPlaylists,
  parsePlaylistUrl,
  saveAppConfig,
} from "../lib/tauri";
import type { AppConfig, PlaylistSummary } from "../lib/types";

type Route =
  | {
      name: "home";
    }
  | {
      name: "playlist";
      playlistId: string;
    };

export function AppShell() {
  const [route, setRoute] = useState<Route>({ name: "home" });
  const [playlists, setPlaylists] = useState<PlaylistSummary[]>([]);
  const [databasePath, setDatabasePath] = useState<string | null>(null);
  const [appConfig, setAppConfig] = useState<AppConfig>({
    youtubeApiKey: "",
    vlcPath: "",
    ytdlpPath: "",
  });
  const [configPath, setConfigPath] = useState<string | null>(null);
  const [configSource, setConfigSource] = useState<string | null>(null);
  const [loadingPlaylists, setLoadingPlaylists] = useState(true);
  const [importBusy, setImportBusy] = useState(false);
  const [configBusy, setConfigBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  useEffect(() => {
    void hydrateLibrary();
  }, []);

  async function hydrateLibrary(): Promise<void> {
    setLoadingPlaylists(true);
    setError(null);

    try {
      await ensureLocalDatabase();
      const databaseStatus = await getDatabaseStatus();
      const currentAppConfig = await getAppConfig();
      const storedPlaylists = await listPlaylists();
      setDatabasePath(databaseStatus.databasePath);
      setAppConfig(currentAppConfig.config);
      setConfigPath(currentAppConfig.configPath);
      setConfigSource(currentAppConfig.configSource);
      setPlaylists(storedPlaylists);
    } catch (nextError) {
      setError(toErrorMessage(nextError));
    } finally {
      setLoadingPlaylists(false);
    }
  }

  async function handleImport(sourceUrl: string): Promise<void> {
    setImportBusy(true);
    setError(null);
    setSuccessMessage(null);

    try {
      await parsePlaylistUrl(sourceUrl);
      const result = await importPlaylist(sourceUrl);
      const storedPlaylists = await listPlaylists();
      setPlaylists(storedPlaylists);
      setSuccessMessage(
        `Playlist importée : ${result.playlist.title} (${result.importedVideoCount} vidéos).`,
      );
    } catch (nextError) {
      setError(toErrorMessage(nextError));
    } finally {
      setImportBusy(false);
    }
  }

  async function handleSaveConfig(config: AppConfig): Promise<void> {
    setConfigBusy(true);
    setError(null);
    setSuccessMessage(null);

    try {
      const saved = await saveAppConfig(config);
      setAppConfig(saved.config);
      setConfigPath(saved.configPath);
      setConfigSource(saved.configSource);
      setSuccessMessage(`Configuration enregistrée dans ${saved.configPath}.`);
    } catch (nextError) {
      setError(toErrorMessage(nextError));
    } finally {
      setConfigBusy(false);
    }
  }

  if (route.name === "playlist") {
    return (
      <PlaylistDetailView
        playlistId={route.playlistId}
        onBack={() => setRoute({ name: "home" })}
        onLibraryChanged={hydrateLibrary}
      />
    );
  }

  return (
    <PlaylistHomeView
      playlists={playlists}
      busy={loadingPlaylists}
      importBusy={importBusy}
      configBusy={configBusy}
      databasePath={databasePath}
      appConfig={appConfig}
      configPath={configPath}
      configSource={configSource}
      error={error}
      successMessage={successMessage}
      onImport={handleImport}
      onSaveConfig={handleSaveConfig}
      onOpenPlaylist={(playlistId) => setRoute({ name: "playlist", playlistId })}
    />
  );
}
