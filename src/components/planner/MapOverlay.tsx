"use client";

// Controls and readouts drawn over the 3D canvas: camera pills + reset (top right), map buttons and the
// live terrain HUD under the cursor (bottom right), tile streaming chip, and a contextual hint (bottom left,
// beside the floating panel).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useState } from "react";
import {
  getSim,
  onSimEvent,
  type CameraEvent,
  type CameraMode,
  type PickTarget,
  type PointInfo,
  type RouteResult,
  type TilesEvent,
} from "@/lib/marsSim";
import { ResetIcon } from "./icons";
import RouteInfoPin from "./RouteInfoPin";
import { chip, fmtLat, fmtLon } from "./ui";

// Drone flies with the keyboard (WASD, Q/E, R/F): PC only, hidden on touch-first devices (phones, tablets).
const MODES: { id: CameraMode; label: string; pcOnly?: boolean }[] = [
  { id: "map", label: "Orbit" },
  { id: "rover", label: "Rover" },
  { id: "drone", label: "Drone", pcOnly: true },
];

const CONTROLS: Record<CameraMode, string> = {
  map: "Drag to move · Scroll or pinch to zoom · Double-click to zoom in · Shift, Ctrl or right-drag to rotate and tilt · Arrows move, +/− zoom",
  drone: "Drag to look · WASD fly · Q/E or ←/→ turn · ↑/↓ look up/down · R/F climb/descend · Shift faster · Scroll changes speed",
  rover: "Drag to orbit the rover · Scroll to zoom · Press Play rover in the panel to drive the route",
};

const pill =
  "flex min-h-9 items-center gap-1.5 rounded-full px-3.5 text-[13px] font-medium transition-colors";

/** Live readout of the terrain under the cursor; placeholders when the cursor is off the map. */
function HoverReadout() {
  const [p, setP] = useState<PointInfo | null>(null);
  useEffect(() => {
    const off = onSimEvent<PointInfo>("mars-hover", setP);
    // The sim stops emitting when the cursor leaves; clear the stale readout ourselves.
    const canvas = document.getElementById("mars-canvas");
    const leave = () => setP(null);
    canvas?.addEventListener("pointerleave", leave);
    return () => {
      off();
      canvas?.removeEventListener("pointerleave", leave);
    };
  }, []);
  const term = "font-mono text-[10px] uppercase tracking-[0.14em] text-scrim-muted";
  return (
    // Mouse-only echo of the terrain under the cursor; picks are announced in the panel instead.
    <dl
      aria-hidden
      className={`${chip} hidden w-60 grid-cols-[auto_1fr] items-baseline gap-x-4 gap-y-1.5 px-3.5 py-3 text-[12px] tabular-nums sm:grid`}
    >
      <dt className={term}>Cursor</dt>
      <dd className="truncate text-right font-mono">{p ? `${fmtLat(p.lat)} ${fmtLon(p.lon)}` : "—"}</dd>
      <dt className={term}>Elevation</dt>
      <dd className="text-right font-mono">{p ? `${p.elevation_m.toFixed(0)} m` : "—"}</dd>
      <dt className={term}>Slope</dt>
      <dd className="flex items-center justify-end gap-2 font-mono">
        {p ? `${p.slope_deg.toFixed(1)}°` : "—"}
        {p && (
          <span
            className={`rounded-full px-1.5 py-px text-[9.5px] font-medium uppercase tracking-wider text-white ${p.passable ? "bg-scrim-ok" : "bg-scrim-danger"}`}
          >
            {p.passable ? "Passable" : "Blocked"}
          </span>
        )}
      </dd>
    </dl>
  );
}

/** Google Maps-style map buttons: compass (north up), zoom in/out, 2D/3D. */
function MapControls() {
  const [cam, setCam] = useState<CameraEvent>({ heading_deg: 0, tilt_deg: 36 });
  useEffect(() => onSimEvent<CameraEvent>("mars-camera", setCam), []);
  const btn = `${chip} pointer-events-auto grid size-10 place-items-center text-[13px] font-semibold transition-colors hover:bg-white/10 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-signal`;
  const inner =
    "grid size-10 place-items-center text-lg transition-colors hover:bg-white/10 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-signal";
  const topDown = cam.tilt_deg < 5;
  return (
    <div className="flex flex-col items-end gap-1.5">
      <button
        type="button"
        onClick={() => getSim()?.map_north()}
        aria-label={`Compass: heading ${Math.round(cam.heading_deg)}°. Turn north up`}
        title="North up"
        className={btn}
      >
        <svg aria-hidden viewBox="0 0 24 24" className="size-5" style={{ transform: `rotate(${-cam.heading_deg}deg)` }}>
          <path d="M12 2.5 15.5 12h-7z" fill="var(--end)" />
          <path d="M12 21.5 8.5 12h7z" fill="currentColor" opacity="0.6" />
        </svg>
      </button>
      <div className={`${chip} pointer-events-auto flex flex-col overflow-hidden`}>
        <button type="button" onClick={() => getSim()?.map_zoom(1)} aria-label="Zoom in" className={inner}>
          +
        </button>
        <span aria-hidden className="mx-2 h-px bg-white/10" />
        <button type="button" onClick={() => getSim()?.map_zoom(-1)} aria-label="Zoom out" className={inner}>
          −
        </button>
      </div>
      <button
        type="button"
        onClick={() => getSim()?.map_top_down(!topDown)}
        aria-label={topDown ? "Switch to tilted 3D view" : "Switch to top-down 2D view"}
        className={btn}
      >
        {topDown ? "3D" : "2D"}
      </button>
    </div>
  );
}

function TilesChip() {
  const [t, setT] = useState<TilesEvent | null>(null);
  useEffect(() => onSimEvent<TilesEvent>("mars-tiles", setT), []);
  if (!t || t.loading <= 0) return null;
  return (
    <p role="status" className={`${chip} flex items-center gap-2 rounded-full px-3 py-1.5 font-mono text-[11px]`}>
      <span aria-hidden className="size-1.5 animate-pulse rounded-full bg-signal" />
      Loading HiRISE detail… ({t.loading})
    </p>
  );
}

export default function MapOverlay({
  ready,
  camera,
  onCamera,
  target,
  keysActive,
  route,
  panelOpen,
}: {
  ready: boolean;
  camera: CameraMode;
  onCamera: (m: CameraMode) => void;
  target: PickTarget;
  keysActive: boolean;
  route: RouteResult | null;
  /** Desktop: the floating panel covers the left 376 px, so left-anchored chips move past it. */
  panelOpen: boolean;
}) {
  if (!ready) return null;
  return (
    <>
      <RouteInfoPin route={route} />

      <div className="pointer-events-none absolute right-2 top-2 flex flex-col items-end gap-2 lg:right-4 lg:top-4">
        <div className={`${chip} pointer-events-auto flex items-center gap-0.5 rounded-full p-1`}>
          <fieldset className="flex gap-0.5">
            <legend className="sr-only">Camera mode</legend>
            {MODES.map((m) => (
              <label key={m.id} className={`cursor-pointer ${m.pcOnly ? "pointer-coarse:hidden" : ""}`}>
                <input
                  type="radio"
                  name="camera"
                  value={m.id}
                  checked={camera === m.id}
                  onChange={() => onCamera(m.id)}
                  className="peer sr-only"
                />
                <span
                  className={`${pill} text-scrim-muted hover:text-scrim-ink peer-checked:bg-scrim-ink peer-checked:text-scrim peer-checked:shadow-sm peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-signal`}
                >
                  {m.label}
                </span>
              </label>
            ))}
          </fieldset>
          <span aria-hidden className="mx-1 h-5 w-px bg-white/10" />
          <button
            type="button"
            onClick={() => getSim()?.reset_view()}
            className={`${pill} text-scrim-muted hover:bg-white/[0.07] hover:text-scrim-ink`}
          >
            <ResetIcon className="size-3.5" />
            <span className="sr-only sm:not-sr-only">Reset view</span>
          </button>
        </div>
        <TilesChip />
      </div>

      <div className="pointer-events-none absolute bottom-2 right-2 flex flex-col items-end gap-2 lg:bottom-4 lg:right-4">
        {camera === "map" && <MapControls />}
        <HoverReadout />
      </div>

      <div
        className={`pointer-events-none absolute bottom-2 left-2 right-16 sm:right-[17rem] lg:bottom-4 ${panelOpen ? "lg:left-[392px]" : "lg:left-4"}`}
      >
        <div className={`${chip} w-fit max-w-[34rem] px-3.5 py-2.5 text-xs leading-relaxed`}>
          <p className="flex items-center gap-2 font-medium">
            <span
              aria-hidden
              className="inline-block size-2 flex-none rounded-full"
              style={{ background: target === "start" ? "var(--start)" : "var(--end)" }}
            />
            Click the terrain to place {target === "start" ? "START" : "END"}
          </p>
          <p className="mt-0.5 hidden text-scrim-ink/75 md:block">{CONTROLS[camera]}</p>
          <p className="hidden text-scrim-muted md:block">
            {keysActive ? "Keyboard active in the 3D view." : "Click the 3D view to use the keyboard."}
          </p>
        </div>
      </div>
    </>
  );
}
