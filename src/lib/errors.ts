import type { AppErrorPayload } from "./types";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isAppErrorPayload(value: unknown): value is AppErrorPayload {
  return (
    isRecord(value) &&
    typeof value.kind === "string" &&
    typeof value.message === "string"
  );
}

export function toErrorMessage(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }

  if (isAppErrorPayload(error)) {
    return error.message;
  }

  if (isRecord(error) && typeof error.kind === "string") {
    return typeof error.message === "string"
      ? error.message
      : `Erreur ${error.kind}.`;
  }

  if (isRecord(error) && isAppErrorPayload(error.error)) {
    return error.error.message;
  }

  if (isRecord(error) && isRecord(error.error) && typeof error.error.kind === "string") {
    return typeof error.error.message === "string"
      ? error.error.message
      : `Erreur ${error.error.kind}.`;
  }

  if (error instanceof Error) {
    return error.message;
  }

  if (isRecord(error)) {
    try {
      return JSON.stringify(error);
    } catch {
      return "Une erreur inattendue est survenue.";
    }
  }

  return "Une erreur inattendue est survenue.";
}
