import type { PlaylistSummary } from "../../../lib/types";

interface PlaylistListProps {
  playlists: PlaylistSummary[];
  onOpenPlaylist: (playlistId: string) => void;
}

function formatDate(value: string | null): string {
  if (!value) {
    return "Jamais synchronisée";
  }

  return new Intl.DateTimeFormat("fr-FR", {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(value));
}

export function PlaylistList({
  playlists,
  onOpenPlaylist,
}: PlaylistListProps) {
  if (playlists.length === 0) {
    return (
      <section className="panel stack gap-sm empty-state">
        <h2>Aucune playlist importée</h2>
        <p className="muted">
          Importez une première playlist pour créer votre bibliothèque locale.
        </p>
      </section>
    );
  }

  return (
    <section className="playlist-grid" aria-label="Playlists importées">
      {playlists.map((playlist) => (
        <button
          key={playlist.id}
          className="playlist-card"
          type="button"
          onClick={() => onOpenPlaylist(playlist.id)}
        >
          {playlist.thumbnailUrl ? (
            <img
              className="playlist-card-thumbnail"
              src={playlist.thumbnailUrl}
              alt=""
            />
          ) : (
            <div className="playlist-card-thumbnail placeholder">
              Miniature indisponible
            </div>
          )}

          <div className="stack gap-xs">
            <div className="stack gap-xxs">
              <h3>{playlist.title}</h3>
              <p className="muted">
                {playlist.channelTitle ?? "Chaîne inconnue"}
              </p>
            </div>

            <div className="playlist-meta">
              <span>{playlist.videoCount} vidéos</span>
              <span>{formatDate(playlist.lastSyncedAt)}</span>
            </div>
          </div>
        </button>
      ))}
    </section>
  );
}
