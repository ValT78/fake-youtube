type TauriInternals = {
  invoke: (
    command: string,
    args?: Record<string, unknown>,
    options?: unknown,
  ) => Promise<unknown>;
};

function getTauriInternals(): TauriInternals | null {
  if (typeof window === "undefined") {
    return null;
  }

  const candidate = (window as Window & {
    __TAURI_INTERNALS__?: TauriInternals;
  }).__TAURI_INTERNALS__;

  return candidate ?? null;
}

export function ensureTauriRuntime(): void {
  if (getTauriInternals() !== null) {
    return;
  }

  throw new Error(
    "Le runtime Tauri n'est pas disponible. Lance l'application desktop avec `npm run dev`, pas uniquement le serveur web.",
  );
}

export async function invokeTauri<TResponse>(
  command: string,
  args?: Record<string, unknown>,
): Promise<TResponse> {
  const tauriInternals = getTauriInternals();

  if (tauriInternals === null) {
    throw new Error(
      "Le runtime Tauri n'est pas disponible. Lance l'application desktop avec `npm run dev`, pas uniquement le serveur web.",
    );
  }

  return tauriInternals.invoke(command, args) as Promise<TResponse>;
}
