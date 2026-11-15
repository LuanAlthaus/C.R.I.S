"use client";

// Controls and readouts drawn over the 3D canvas: camera switcher + reset (top), live terrain
// readout under the cursor with a contextual hint (bottom-left), tile streaming chip (bottom-right).
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
import RouteInfoPin from "./RouteInfoPin";
import { chip, fmtLat, fmtLon } from "./ui";

const MODES: { id: CameraMode; label: string }[] = [
  { id: "map", label: "Map 3D" },
  { id: "drone", label: "Drone" },
  { id: "rover", label: "Rover" },
];

const CONTROLS: Record<CameraMode, string> = {
  map: "Drag to move · Scroll or pinch to zoom · Double-click to zoom in · Shift, Ctrl or right-drag to rotate and tilt · Arrows move, +/− zoom",
  drone: "Drag to look · WASD fly · Q/E or ←/→ turn · ↑/↓ look up/down · R/F climb/descend · Shift faster · Scroll changes speed",
  rover: "Drag to orbit the rover · Scroll to zoom · Press Play in the panel to drive the route",
};

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
  if (!p) return null;
  return (
    // Mouse-only echo of the terrain under the cursor; picks are announced in the panel instead.
    <dl
      aria-hidden
      className={`${chip} grid w-max grid-cols-[auto_auto] gap-x-3 gap-y-0.5 px-3 py-2 font-mono text-[11px] tabular-nums`}
    >
      <dt className="text-scrim-muted">Lat / lon</dt>
      <dd>
        {fmtLat(p.lat)} {fmtLon(p.lon)}
      </dd>
      <dt className="text-scrim-muted">Elevation</dt>
      <dd>{p.elevation_m.toFixed(0)} m</dd>
      <dt className="text-scrim-muted">Slope</dt>
      <dd className="flex items-center gap-2">
        {p.slope_deg.toFixed(1)}°
        <span
          className={`rounded px-1.5 text-[10px] uppercase tracking-wider text-white ${p.passable ? "bg-scrim-ok" : "bg-scrim-danger"}`}
        >
          {p.passable ? "Walkable" : "Blocked"}
        </span>
      </dd>
    </dl>
  );
}

/** Google Maps-style map buttons: compass (north up), zoom in/out, 2D/3D. */
function MapControls() {
  const [cam, setCam] = useState<CameraEvent>({ heading_deg: 0, tilt_deg: 36 });
  useEffect(() => onSimEvent<CameraEvent>("mars-camera", setCam), []);
  const btn = `${chip} pointer-events-auto grid size-11 place-items-center text-sm font-semibold transition-colors hover:border-white/40 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-signal`;
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
        <svg aria-hidden viewBox="0 0 24 24" className="size-6" style={{ transform: `rotate(${-cam.heading_deg}deg)` }}>
          <path d="M12 2.5 15.5 12h-7z" fill="#d6452f" />
          <path d="M12 21.5 8.5 12h7z" fill="currentColor" opacity="0.7" />
        </svg>
      </button>
      <div className={`${chip} pointer-events-auto flex flex-col overflow-hidden`}>
        <button
          type="button"
          onClick={() => getSim()?.map_zoom(1)}
          aria-label="Zoom in"
          className="grid size-11 place-items-center text-lg transition-colors hover:bg-white/10 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-signal"
        >
          +
        </button>
        <span aria-hidden className="mx-2 h-px bg-white/15" />
        <button
          type="button"
          onClick={() => getSim()?.map_zoom(-1)}
          aria-label="Zoom out"
          className="grid size-11 place-items-center text-lg transition-colors hover:bg-white/10 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-signal"
        >
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
    <p role="status" className={`${chip} flex items-center gap-2 px-2.5 py-1.5 font-mono text-[11px]`}>
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
}: {
  ready: boolean;
  camera: CameraMode;
  onCamera: (m: CameraMode) => void;
  target: PickTarget;
  keysActive: boolean;
  route: RouteResult | null;
}) {
  if (!ready) return null;
  return (
    <>
      <RouteInfoPin route={route} />
      <div className="pointer-events-none absolute inset-x-2 top-2 flex flex-wrap items-start justify-between gap-2 lg:inset-x-4 lg:top-4">
        <fieldset className={`${chip} pointer-events-auto flex gap-1 p-1`}>
          <legend className="sr-only">Camera mode</legend>
          {MODES.map((m) => (
            <label key={m.id} className="cursor-pointer">
              <input
                type="radio"
                name="camera"
                value={m.id}
                checked={camera === m.id}
                onChange={() => onCamera(m.id)}
                className="peer sr-only"
              />
              <span className="flex min-h-11 items-center rounded px-3 text-sm font-medium text-scrim-muted transition-colors hover:text-scrim-ink peer-checked:bg-scrim-ink peer-checked:text-scrim peer-checked:hover:text-scrim peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-signal">
                {m.label}
              </span>
            </label>
          ))}
        </fieldset>
        <button
          type="button"
          onClick={() => getSim()?.reset_view()}
          className={`${chip} pointer-events-auto inline-flex min-h-11 items-center gap-2 px-3 text-sm font-medium transition-colors hover:border-white/40`}
        >
          <svg aria-hidden viewBox="0 0 16 16" className="size-4">
            <path
              d="M3 8a5 5 0 1 0 1.6-3.7M3 2.5v2.8h2.8"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
          Reset view
        </button>
      </div>

      <div className="pointer-events-none absolute inset-x-2 bottom-2 flex items-end justify-between gap-2 lg:inset-x-4 lg:bottom-4">
        <div className="min-w-0 space-y-1.5">
          <HoverReadout />
          <div className={`${chip} max-w-[34rem] px-3 py-2 text-xs leading-relaxed`}>
            <p className="font-medium">
              <span
                aria-hidden
                className="mr-1.5 inline-block size-2 rounded-full"
                style={{ background: target === "start" ? "#2e9e5b" : "#d6452f" }}
              />
              Click the terrain to place {target === "start" ? "START" : "END"}
            </p>
            <p className="text-scrim-ink/80">{CONTROLS[camera]}</p>
            <p className="text-scrim-muted">
              {keysActive ? "Keyboard active in the 3D view." : "Click the 3D view to use the keyboard."}
            </p>
          </div>
        </div>
        <div className="flex flex-col items-end gap-1.5">
          {camera === "map" && <MapControls />}
          <TilesChip />
        </div>
      </div>
    </>
  );
}
