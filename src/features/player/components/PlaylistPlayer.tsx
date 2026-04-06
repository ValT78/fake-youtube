import type { PlaylistVideoItem } from "../../../lib/types";

interface PlaylistPlayerProps {
  playlistTitle: string;
  currentVideo: PlaylistVideoItem | null;
  canGoPrevious: boolean;
  canGoNext: boolean;
  launchBusy: boolean;
  onLaunch: () => void;
  onPrevious: () => void;
  onNext: () => void;
}

export function PlaylistPlayer({
  playlistTitle,
  currentVideo,
  canGoPrevious,
  canGoNext,
  launchBusy,
  onLaunch,
  onPrevious,
  onNext,
}: PlaylistPlayerProps) {
  if (!currentVideo) {
    return (
      <section className="panel stack gap-sm player-empty">
        <h2>{playlistTitle}</h2>
        <p className="muted">Cette playlist ne contient pas encore de vidéo lisible.</p>
      </section>
    );
  }

  return (
    <section className="player-panel stack gap-sm">
      <div className="player-preview">
        {currentVideo.thumbnailUrl ? (
          <img
            className="player-preview-thumbnail"
            src={currentVideo.thumbnailUrl}
            alt=""
          />
        ) : (
          <div className="player-preview-thumbnail placeholder">
            Aperçu indisponible
          </div>
        )}
      </div>

      <div className="stack gap-xs">
        <div className="stack gap-xxs">
          <p className="eyebrow">Lecture en cours</p>
          <h2>{currentVideo.title}</h2>
          <p className="muted">
            {currentVideo.channelTitle ?? "Chaîne inconnue"}
          </p>
          <p className="muted">
            Cette application ouvre maintenant la vidéo dans VLC au lieu de la
            lire dans la fenêtre Tauri.
          </p>
        </div>

        <div className="player-controls">
          <button
            className="button button-primary"
            type="button"
            onClick={onLaunch}
            disabled={launchBusy}
          >
            {launchBusy ? "Ouverture de VLC..." : "Lire dans VLC"}
          </button>
          <button
            className="button"
            type="button"
            onClick={onPrevious}
            disabled={!canGoPrevious}
          >
            Précédente
          </button>
          <button
            className="button"
            type="button"
            onClick={onNext}
            disabled={!canGoNext}
          >
            Suivante
          </button>
        </div>
      </div>
    </section>
  );
}
