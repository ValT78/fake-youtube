import { FormEvent, useEffect, useState } from "react";
import type { AppConfig } from "../../../lib/types";

interface AppConfigFormProps {
  busy: boolean;
  config: AppConfig;
  configPath: string | null;
  configSource: string | null;
  onSubmit: (config: AppConfig) => Promise<void>;
}

function describeConfigSource(configSource: string | null): string {
  switch (configSource) {
    case "executable":
      return "Le JSON situé à côté du .exe est utilisé en priorité.";
    case "appData":
      return "Le JSON enregistré par l'application est actuellement utilisé.";
    default:
      return "Aucun fichier trouvé pour l'instant. Enregistre ce formulaire pour créer le JSON.";
  }
}

export function AppConfigForm({
  busy,
  config,
  configPath,
  configSource,
  onSubmit,
}: AppConfigFormProps) {
  const [draft, setDraft] = useState<AppConfig>(config);

  useEffect(() => {
    setDraft(config);
  }, [config]);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await onSubmit(draft);
  }

  return (
    <form className="panel stack gap-sm" onSubmit={handleSubmit}>
      <div className="stack gap-xs">
        <label className="label" htmlFor="youtube-api-key">
          Configuration JSON
        </label>
        <p className="muted">
          Remplis ces champs puis enregistre. L'application stocke un fichier{" "}
          <code>playlist-browser.config.json</code>.
        </p>
        <p className="muted small">{describeConfigSource(configSource)}</p>
        {configPath ? (
          <p className="muted small">
            Chemin du JSON : <code>{configPath}</code>
          </p>
        ) : null}
      </div>

      <div className="stack gap-xs">
        <label className="label" htmlFor="youtube-api-key">
          Clé YouTube API
        </label>
        <input
          id="youtube-api-key"
          className="input"
          type="text"
          value={draft.youtubeApiKey}
          onChange={(event) =>
            setDraft((current) => ({
              ...current,
              youtubeApiKey: event.target.value,
            }))
          }
          placeholder="AIza..."
          autoComplete="off"
          spellCheck={false}
          disabled={busy}
        />
      </div>

      <div className="stack gap-xs">
        <label className="label" htmlFor="vlc-path">
          Chemin VLC
        </label>
        <input
          id="vlc-path"
          className="input"
          type="text"
          value={draft.vlcPath}
          onChange={(event) =>
            setDraft((current) => ({
              ...current,
              vlcPath: event.target.value,
            }))
          }
          placeholder="Laisse vide pour utiliser l'auto-détection"
          autoComplete="off"
          spellCheck={false}
          disabled={busy}
        />
      </div>

      <div className="stack gap-xs">
        <label className="label" htmlFor="ytdlp-path">
          Chemin yt-dlp
        </label>
        <input
          id="ytdlp-path"
          className="input"
          type="text"
          value={draft.ytdlpPath}
          onChange={(event) =>
            setDraft((current) => ({
              ...current,
              ytdlpPath: event.target.value,
            }))
          }
          placeholder="Laisse vide pour utiliser l'auto-détection"
          autoComplete="off"
          spellCheck={false}
          disabled={busy}
        />
      </div>

      <div className="form-row">
        <button className="button button-primary" type="submit" disabled={busy}>
          {busy ? "Enregistrement..." : "Enregistrer le JSON"}
        </button>
      </div>
    </form>
  );
}
