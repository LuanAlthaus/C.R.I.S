// Shared bits for the planner panel: the collapsible row frame, one field/button look, number formatting.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { ReactNode } from "react";
import { ChevronIcon } from "./icons";

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

export const labelCls = "font-mono text-[10.5px] uppercase tracking-[0.16em] text-muted";
export const inputCls =
  "h-10 w-full rounded-lg border border-line bg-bg px-3 font-mono text-sm tabular-nums text-ink transition-colors hover:border-white/20 user-invalid:border-danger";
export const buttonCls =
  "inline-flex min-h-11 items-center justify-center gap-2 rounded-xl border border-line bg-white/[0.06] px-4 text-sm font-medium text-ink transition-[background-color,border-color,transform] duration-150 enabled:hover:bg-white/[0.11] enabled:active:translate-y-px disabled:cursor-not-allowed disabled:opacity-40";
export const primaryCls =
  "inline-flex min-h-11 items-center justify-center gap-2 rounded-xl bg-gold px-4 text-sm font-semibold text-gold-ink shadow-[0_8px_24px_-10px_var(--gold)] transition-[filter,transform,box-shadow] duration-150 enabled:hover:brightness-110 enabled:active:translate-y-px disabled:cursor-not-allowed disabled:opacity-40 disabled:shadow-none";
export const citeCls = "text-xs leading-relaxed text-muted";
/** Translucent card drawn over the 3D canvas. */
export const chip =
  "rounded-xl border border-white/[0.08] bg-scrim/85 text-scrim-ink shadow-[0_10px_30px_-12px_rgba(0,0,0,0.8)] backdrop-blur-md";

/**
 * One collapsible row of the panel: icon, name, a short summary on the right and a chevron.
 * Native <details>, so the content stays mounted (fetches and uncontrolled inputs keep their state).
 */
export function Collapsible({
  id,
  icon,
  title,
  summary,
  defaultOpen = false,
  children,
}: {
  id: string;
  icon: ReactNode;
  title: string;
  summary?: ReactNode;
  defaultOpen?: boolean;
  children: ReactNode;
}) {
  return (
    <details open={defaultOpen} className="group/row border-t border-line">
      <summary className="flex min-h-12 cursor-pointer list-none items-center gap-3 px-4 py-2 transition-colors hover:bg-white/[0.03] focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-signal [&::-webkit-details-marker]:hidden">
        <span
          aria-hidden
          className="grid size-7 flex-none place-items-center rounded-lg bg-white/[0.05] text-muted transition-colors group-open/row:bg-signal-soft group-open/row:text-signal"
        >
          {icon}
        </span>
        <span id={id} className="min-w-0 flex-1 truncate text-[13.5px] font-medium">
          {title}
        </span>
        {summary != null && (
          <span className="max-w-[48%] truncate font-mono text-[11px] tabular-nums text-muted">{summary}</span>
        )}
        <ChevronIcon className="size-3 flex-none text-muted transition-transform duration-200 group-open/row:rotate-90" />
      </summary>
      <div role="group" aria-labelledby={id} className="space-y-3.5 px-4 pb-5 pt-1.5">
        {children}
      </div>
    </details>
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
