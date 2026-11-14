// Shared bits for the planner panel: one section frame, one field/button look, number formatting.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { ReactNode } from "react";

/** Subset of public/data/jezero/meta.json the UI reads. */
export interface SiteMeta {
  site: string;
  center: { lat: number; lon: number };
  geology_units: { id: number; label: string }[];
  hazard_model: { type: string };
  radiation: { type: string };
  thermal: { type: string };
  credits: { layer: string; text: string }[];
}

export const labelCls = "font-mono text-[11px] uppercase tracking-[0.14em] text-muted";
export const inputCls =
  "h-11 w-full rounded-md border border-line bg-bg px-3 font-mono text-sm tabular-nums text-ink user-invalid:border-danger";
export const buttonCls =
  "inline-flex min-h-11 items-center justify-center gap-2 rounded-md border border-line bg-bg px-4 text-sm font-medium text-ink transition-[border-color,transform] duration-150 enabled:hover:border-ink enabled:active:translate-y-px disabled:cursor-not-allowed disabled:opacity-45";
export const primaryCls =
  "inline-flex min-h-11 items-center justify-center gap-2 rounded-md border border-ink bg-ink px-4 text-sm font-medium text-bg transition-[background-color,border-color,transform] duration-150 enabled:hover:border-signal enabled:hover:bg-signal enabled:active:translate-y-px disabled:cursor-not-allowed disabled:opacity-45";
export const citeCls = "text-xs leading-relaxed text-muted";

export function Section({ n, id, title, children }: { n: string; id: string; title: string; children: ReactNode }) {
  return (
    <section aria-labelledby={id} className="border-b border-line px-4 py-5 last:border-b-0">
      <h2 id={id} className="flex items-baseline gap-2.5 font-mono text-[11px] uppercase tracking-[0.18em] text-muted">
        <span className="text-signal">{n}</span>
        {title}
      </h2>
      <div className="mt-3.5 space-y-3.5">{children}</div>
    </section>
  );
}

export const fmtLat = (lat: number) => `${Math.abs(lat).toFixed(4)}° ${lat >= 0 ? "N" : "S"}`;
export const fmtLon = (lon: number) => `${Math.abs(lon).toFixed(4)}° ${lon >= 0 ? "E" : "W"}`;
export const fmtLength = (m: number) => (m < 1000 ? `${Math.round(m)} m` : `${(m / 1000).toFixed(2)} km`);
export function fmtEta(min: number) {
  if (min < 60) return `${Math.round(min)} min`;
  const h = Math.floor(min / 60);
  return `${h} h ${String(Math.round(min - h * 60)).padStart(2, "0")} min`;
}
