import { useEffect, useState } from "react";
import { toErrorMessage } from "../../lib/errors";
import {
  getPlaylistDetail,
  importPlaylist,
  openVideoInVlc,
} from "../../lib/tauri";
import type { PlaylistDetail } from "../../lib/types";
import { PlaylistPlayer } from "../player/components/PlaylistPlayer";
import { VideoQueue } from "../player/components/VideoQueue";
import {
  getNextVideoIndex,
  getPreviousVideoIndex,
} from "../player/playerState";

interface PlaylistDetailViewProps {
  playlistId: string;
  onBack: () => void;
  onLibraryChanged: () => Promise<void>;
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

export function PlaylistDetailView({
  playlistId,
  onBack,
  onLibraryChanged,
}: PlaylistDetailViewProps) {
  const [detail, setDetail] = useState<PlaylistDetail | null>(null);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [loading, setLoading] = useState(true);
  const [syncing, setSyncing] = useState(false);
  const [launchingVideo, setLaunchingVideo] = useState(false);
  const [pageError, setPageError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    async function loadDetail(): Promise<void> {
      setLoading(true);
      setPageError(null);

      try {
        const nextDetail = await getPlaylistDetail(playlistId);

        if (cancelled) {
          return;
        }

        setDetail(nextDetail);
        setCurrentIndex(0);
      } catch (nextError) {
        if (!cancelled) {
          setPageError(toErrorMessage(nextError));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadDetail();

    return () => {
      cancelled = true;
    };
  }, [playlistId]);

  async function handleSync(): Promise<void> {
    if (!detail) {
      return;
    }

    setSyncing(true);
    setActionError(null);

    try {
      await importPlaylist(detail.sourceUrl);
      await onLibraryChanged();
      const refreshedDetail = await getPlaylistDetail(playlistId);
      setDetail(refreshedDetail);
      setCurrentIndex(0);
    } catch (nextError) {
      setActionError(toErrorMessage(nextError));
    } finally {
      setSyncing(false);
    }
  }

  async function handleLaunchCurrentVideo(): Promise<void> {
    if (!detail) {
      return;
    }

    const currentVideo = detail.videos[currentIndex];

    if (!currentVideo) {
      return;
    }

    setLaunchingVideo(true);
    setActionError(null);

    try {
      await openVideoInVlc(currentVideo.youtubeVideoId);
    } catch (nextError) {
      setActionError(toErrorMessage(nextError));
    } finally {
      setLaunchingVideo(false);
    }
  }

  if (loading) {
    return (
      <main className="app-shell">
        <section className="panel">
          <p>Chargement de la playlist...</p>
        </section>
      </main>
    );
  }

  if (pageError) {
    return (
      <main className="app-shell">
        <section className="panel stack gap-sm">
          <button className="button" type="button" onClick={onBack}>
            Retour
          </button>
          <div className="banner banner-error">{pageError}</div>
        </section>
      </main>
    );
  }

  if (!detail) {
    return (
      <main className="app-shell">
        <section className="panel stack gap-sm">
          <button className="button" type="button" onClick={onBack}>
            Retour
          </button>
          <p>Playlist introuvable.</p>
        </section>
      </main>
    );
  }

  const currentVideo = detail.videos[currentIndex] ?? null;
  const canGoPrevious = currentIndex > 0;
  const canGoNext = currentIndex < detail.videos.length - 1;

  return (
    <main className="app-shell">
      <section className="stack gap-sm">
        <div className="detail-header">
          <button className="button" type="button" onClick={onBack}>
            Retour à l’accueil
          </button>
          <button
            className="button"
            type="button"
            onClick={() => {
              void handleSync();
            }}
            disabled={syncing}
          >
            {syncing ? "Synchronisation..." : "Synchroniser"}
          </button>
        </div>

        <div className="panel stack gap-sm">
          <div className="stack gap-xxs">
            <p className="eyebrow">Playlist</p>
            <h1>{detail.title}</h1>
            <p className="muted">
              {detail.channelTitle ?? "Chaîne inconnue"} · {detail.videoCount} vidéos
            </p>
          </div>

          {detail.description ? <p>{detail.description}</p> : null}

          <p className="muted small">
            Dernière synchro : {formatDate(detail.lastSyncedAt)}
          </p>
        </div>

        {actionError ? <div className="banner banner-error">{actionError}</div> : null}

        <section className="detail-layout">
          <PlaylistPlayer
            playlistTitle={detail.title}
            currentVideo={currentVideo}
            canGoPrevious={canGoPrevious}
            canGoNext={canGoNext}
            launchBusy={launchingVideo}
            onLaunch={() => {
              void handleLaunchCurrentVideo();
            }}
            onPrevious={() => setCurrentIndex(getPreviousVideoIndex(currentIndex))}
            onNext={() =>
              setCurrentIndex(getNextVideoIndex(currentIndex, detail.videos.length))
            }
          />

          <VideoQueue
            videos={detail.videos}
            currentIndex={currentIndex}
            onSelect={setCurrentIndex}
          />
        </section>
      </section>
    </main>
  );
}
