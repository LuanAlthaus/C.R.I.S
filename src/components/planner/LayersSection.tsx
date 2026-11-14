"use client";

// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { OVERLAY_LEGENDS, type Overlay } from "@/lib/marsSim";
import { citeCls, labelCls, Section, type SiteMeta } from "./ui";

const ORDER: Overlay[] = ["none", "slope", "rough", "texture", "hazard", "thermal", "radiation", "geology", "cost"];
const META_KEY = { hazard: "hazard_model", radiation: "radiation", thermal: "thermal" } as const;
// For these overlays the last legend stop is a separate class (too steep / impassable), not part of the ramp.
const SPLIT_LAST: Overlay[] = ["slope", "hazard", "cost"];

function ramp(o: Overlay) {
  const stops = OVERLAY_LEGENDS[o].stops;
  if (o === "none" || o === "geology") return stops.length ? `linear-gradient(90deg, ${stops.join(", ")})` : "none";
  const g = SPLIT_LAST.includes(o) ? stops.slice(0, -1) : stops;
  // Hazard alpha grows with the score, so its ramp starts transparent.
  return `linear-gradient(90deg, ${(g.length === 1 ? [`${g[0]}00`, g[0]] : g).join(", ")})`;
}

function rampLabels(o: Overlay, allowed: number): string[] {
  switch (o) {
    case "slope":
      return ["0°", `${(allowed / 2).toFixed(1)}°`, `${allowed.toFixed(1)}°`];
    case "rough":
      return ["0 m", "≥1 m"];
    case "texture":
      return ["Smooth", "Speckled"];
    case "hazard":
      return ["Smooth", "Rough"];
    case "thermal":
      return ["Low swing", "High swing"];
    case "radiation":
      return ["Sheltered", "Open sky"];
    case "cost":
      return ["×1", "×6"];
    default:
      return [];
  }
}

const EXTRA_CLASS: Partial<Record<Overlay, (allowed: number) => string>> = {
  slope: (a) => `Too steep (> ${a.toFixed(1)}°)`,
  hazard: () => "Impassable",
  cost: () => "Impassable",
};

// sim/src/layers.rs colours unit `id` with GEOLOGY[id % 12]; OVERLAY_LEGENDS.geology holds GEOLOGY[1..=6].
const unitColor = (id: number) => OVERLAY_LEGENDS.geology.stops[(id % 12) - 1] ?? "var(--muted)";

export default function LayersSection({
  overlay,
  onChange,
  meta,
  allowedSlope,
  showTraverse,
  onShowTraverse,
  vscale,
  onVscale,
}: {
  overlay: Overlay;
  onChange: (o: Overlay) => void;
  meta: SiteMeta | null;
  allowedSlope: number;
  showTraverse: boolean;
  onShowTraverse: (v: boolean) => void;
  vscale: number;
  onVscale: (v: number) => void;
}) {
  const pending = (o: Overlay) => {
    const key = META_KEY[o as keyof typeof META_KEY];
    return !!(key && meta?.[key].type === "none");
  };
  const legend = OVERLAY_LEGENDS[overlay];
  const labels = rampLabels(overlay, allowedSlope);
  const extra = EXTRA_CLASS[overlay];

  return (
    <Section n="03" id="sec-layers" title="Layers">
      <fieldset>
        <legend className="sr-only">Map overlay</legend>
        <div className="divide-y divide-line overflow-hidden rounded-lg border border-line">
          {ORDER.map((o) => (
            <label
              key={o}
              className="flex min-h-11 cursor-pointer items-center gap-3 px-3 py-2 text-sm transition-colors hover:bg-bg has-checked:bg-bg has-focus-visible:outline-2 has-focus-visible:-outline-offset-2 has-focus-visible:outline-signal"
            >
              <input
                type="radio"
                name="overlay"
                value={o}
                checked={overlay === o}
                onChange={() => onChange(o)}
                className="size-4 flex-none accent-signal"
              />
              <span
                aria-hidden
                className="h-2 w-7 flex-none rounded-sm border border-line"
                style={{ backgroundImage: ramp(o) }}
              />
              <span className="min-w-0 flex-1">{OVERLAY_LEGENDS[o].label}</span>
              {pending(o) && (
                <span className="flex-none font-mono text-[10px] uppercase tracking-wider text-muted">
                  not computed yet
                </span>
              )}
            </label>
          ))}
        </div>
      </fieldset>

      <div aria-live="polite" className="space-y-2.5">
        {overlay === "none" ? (
          <p className="text-sm text-muted">HiRISE orthophoto draped on the terrain, no data overlay.</p>
        ) : pending(overlay) ? (
          <p className="rounded-md border border-dashed border-line px-3 py-2 text-sm text-muted">
            {legend.label}: not computed yet. The layer is transparent until the team runs this pipeline stage.
          </p>
        ) : overlay === "geology" ? (
          <ul className="space-y-1.5 text-sm">
            {(meta?.geology_units ?? []).map((u) => (
              <li key={u.id} className="flex items-center gap-2.5">
                <span aria-hidden className="size-3 flex-none rounded-sm" style={{ background: unitColor(u.id) }} />
                {u.label}
              </li>
            ))}
          </ul>
        ) : (
          <div>
            <div aria-hidden className="h-2.5 rounded-sm border border-line" style={{ backgroundImage: ramp(overlay) }} />
            <div className="mt-1 flex justify-between font-mono text-[11px] tabular-nums text-muted">
              {labels.map((l) => (
                <span key={l}>{l}</span>
              ))}
            </div>
            {extra && (
              <p className="mt-2 flex items-center gap-2 text-sm">
                <span
                  aria-hidden
                  className="size-3 flex-none rounded-sm"
                  style={{ background: legend.stops[legend.stops.length - 1] }}
                />
                {extra(allowedSlope)}
              </p>
            )}
          </div>
        )}
        {overlay !== "none" && (
          <>
            <p className="text-xs text-muted">{legend.unit}</p>
            <p className={citeCls}>
              <span className={labelCls}>Source</span> {legend.source}
            </p>
          </>
        )}
      </div>

      <label className="flex min-h-11 cursor-pointer items-center gap-3 rounded-lg border border-line px-3 py-2 text-sm has-focus-visible:outline-2 has-focus-visible:-outline-offset-2 has-focus-visible:outline-signal">
        <input
          type="checkbox"
          checked={showTraverse}
          onChange={(e) => onShowTraverse(e.currentTarget.checked)}
          className="size-4 flex-none accent-signal"
        />
        <span aria-hidden className="h-0.5 w-7 flex-none rounded-full bg-[#8fb8ff]" />
        <span className="min-w-0 flex-1">
          Perseverance traverse
          <span className="block text-xs text-muted">Rover path, sols 13–866 (NASA/JPL-Caltech)</span>
        </span>
      </label>

      <div>
        <div className="flex items-baseline justify-between gap-3">
          <label htmlFor="vscale" className="text-sm">
            Vertical exaggeration
          </label>
          <output htmlFor="vscale" className="font-mono text-xs tabular-nums">
            {vscale}×
          </output>
        </div>
        <input
          id="vscale"
          type="range"
          min={1}
          max={5}
          step={0.5}
          value={vscale}
          onChange={(e) => onVscale(e.currentTarget.valueAsNumber)}
          aria-describedby="vscale-note"
          className="h-11 w-full cursor-pointer accent-signal"
        />
        <p id="vscale-note" className="-mt-2 text-xs text-muted">
          {vscale === 1
            ? "True relief. Analysis always uses true elevations."
            : `Relief exaggerated ${vscale}× for visibility; analysis uses true elevations.`}
        </p>
      </div>
    </Section>
  );
}
