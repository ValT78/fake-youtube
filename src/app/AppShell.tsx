import { useEffect, useState } from "react";
import { PlaylistDetailView } from "../features/playlists/PlaylistDetailView";
import { PlaylistHomeView } from "../features/playlists/PlaylistHomeView";
import { ensureLocalDatabase } from "../lib/db";
import { toErrorMessage } from "../lib/errors";
import {
  getDatabaseStatus,
  importPlaylist,
  listPlaylists,
  parsePlaylistUrl,
} from "../lib/tauri";
import type { PlaylistSummary } from "../lib/types";

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
  const [loadingPlaylists, setLoadingPlaylists] = useState(true);
  const [importBusy, setImportBusy] = useState(false);
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
      const storedPlaylists = await listPlaylists();
      setDatabasePath(databaseStatus.databasePath);
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
      databasePath={databasePath}
      error={error}
      successMessage={successMessage}
      onImport={handleImport}
      onOpenPlaylist={(playlistId) => setRoute({ name: "playlist", playlistId })}
    />
  );
}
