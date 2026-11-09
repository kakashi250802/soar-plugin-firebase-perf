/**
 * Firebase Performance Monitoring (soar-plugin-firebase-perf). Traces are named; a trace runs from `startTrace` to
 * `stopTrace` with the same name. Without a Firebase project the calls do nothing and log "Firebase not configured"
 * once.
 */
declare module "native:perf" {
  /** True when a real Firebase project is configured (not the placeholder config). */
  export function configured(): boolean;
  /** Starts the trace `name` (letters, digits and `_`, up to 100 characters, no leading `_`). Starting a running
   * trace again does nothing. */
  export function startTrace(name: string): void;
  /** Stops the trace `name` and sends it; a trace that is not running is ignored. */
  export function stopTrace(name: string): void;
  /** Adds `by` to the metric `metric` of the running trace `trace`. */
  export function incrementMetric(trace: string, metric: string, by: number): void;
  /** Sets the attribute `key` of the running trace `trace` (up to 5 per trace; key up to 40, value up to 100
   * characters). */
  export function putAttribute(trace: string, key: string, value: string): void;
  /** Turns collection on or off (for a consent switch). */
  export function setEnabled(enabled: boolean): void;
}
