/**
 * Message lisible à partir d'une erreur de `invoke()`.
 *
 * Tauri rejette avec la valeur sérialisée par la commande Rust : ici une
 * `string` en français, pas une `Error`. On garde donc ce texte tel quel, et
 * on ne retombe sur `fallback` que si rien d'exploitable n'est fourni.
 */
export const toErrorMessage = (err: unknown, fallback: string): string => {
  if (typeof err === "string" && err.trim().length > 0) return err;
  if (err instanceof Error && err.message.length > 0) return err.message;
  return fallback;
};
