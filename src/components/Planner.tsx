"use client";

// C.R.I.S shell: planner panel beside the 3D sim on desktop (380 px + canvas), stacked
// under it on mobile. Holds the UI state the sim needs and replays it on ready.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useState } from "react";
import MarsSim from "@/components/MarsSim";
import ConditionsSection from "@/components/planner/ConditionsSection";
import LayersSection from "@/components/planner/LayersSection";
import LimitsSection from "@/components/planner/LimitsSection";
import MapOverlay from "@/components/planner/MapOverlay";
import RawImagesSection, { type NearestStop } from "@/components/planner/RawImagesSection";
import RouteSection from "@/components/planner/RouteSection";
import SourcesFooter from "@/components/planner/SourcesFooter";
import type { SiteMeta } from "@/components/planner/ui";
import {
  DEFAULT_PARAMS,
  getSim,
  onSimEvent,
  type CameraMode,
  type Overlay,
  type PickEvent,
  type PickTarget,
  type RouteParams,
  type RouteResult,
} from "@/lib/marsSim";

// Mirrors weight_factor / s_max_eff in sim/src/route.rs so the UI can show the limit before a route exists.
function allowedSlope(p: RouteParams) {
  const wf = ((p.body_kg + p.suit_kg + p.payload_kg) * 3.71) / (p.body_kg * 9.81);
  return p.s_max_base_deg / Math.max(wf, 1);
}

export default function Planner() {
  const [open, setOpen] = useState(true);
  const [ready, setReady] = useState(false);
  const [camera, setCamera] = useState<CameraMode>("map");
  const [overlay, setOverlay] = useState<Overlay>("none");
  const [params, setParams] = useState<RouteParams>(DEFAULT_PARAMS);
  const [vscale, setVscale] = useState(1);
  const [showTraverse, setShowTraverse] = useState(false);
  const [target, setTarget] = useState<PickTarget>("start");
  const [start, setStart] = useState<PickEvent | null>(null);
  const [end, setEnd] = useState<PickEvent | null>(null);
  const [route, setRoute] = useState<RouteResult | null>(null);
  const [playing, setPlaying] = useState(false);
  const [meta, setMeta] = useState<SiteMeta | null>(null);
  const [near, setNear] = useState<NearestStop | null>(null);
  const [sol, setSol] = useState(1);
  const [keysActive, setKeysActive] = useState(false);

  useEffect(() => {
    fetch("/data/jezero/meta.json")
      .then((r) => (r.ok ? r.json() : null))
      .then(setMeta)
      .catch(() => setMeta(null));
  }, []);

  // Commands sent before the WASM module loads are dropped, so push the current UI state once it is ready.
  useEffect(
    () =>
      onSimEvent("mars-sim-ready", () => {
        setReady(true);
        const sim = getSim();
        sim?.set_camera_mode(camera);
        sim?.set_overlay(overlay);
        sim?.set_params(JSON.stringify(params));
        sim?.set_vertical_scale(vscale);
        sim?.set_show_traverse(showTraverse);
      }),
    [camera, overlay, params, vscale, showTraverse],
  );

  useEffect(() => {
    const t = setTimeout(() => getSim()?.set_params(JSON.stringify(params)), 300);
    return () => clearTimeout(t);
  }, [params]);

  useEffect(() => {
    const offPick = onSimEvent<PickEvent>("mars-pick", (p) => {
      // Moving the start keeps the end; the sim re-plans and emits mars-route.
      (p.which === "start" ? setStart : setEnd)(p);
      setNear({ which: p.which, sol: p.nearest_sol, dist_m: p.nearest_sol_dist_m });
      if (p.nearest_sol !== null) setSol(p.nearest_sol);
    });
    const offTarget = onSimEvent<{ target: PickTarget }>("mars-pick-target", (d) => setTarget(d.target));
    const offRoute = onSimEvent<RouteResult>("mars-route", (r) => {
      setRoute(r);
      setPlaying(false); // the sim rewinds the rover on every new route
    });
    const offStopped = onSimEvent("mars-rover-stopped", () => setPlaying(false));
    return () => {
      offPick();
      offTarget();
      offRoute();
      offStopped();
    };
  }, []);

  const selectCamera = (m: CameraMode) => {
    setCamera(m);
    getSim()?.set_camera_mode(m);
  };
  const selectOverlay = (o: Overlay) => {
    setOverlay(o);
    getSim()?.set_overlay(o);
  };
  const selectTarget = (t: PickTarget) => {
    setTarget(t);
    getSim()?.set_pick_target(t);
  };
  const changeVscale = (v: number) => {
    setVscale(v);
    getSim()?.set_vertical_scale(v);
  };
  const changeTraverse = (v: boolean) => {
    setShowTraverse(v);
    getSim()?.set_show_traverse(v);
  };
  const togglePlay = () => {
    getSim()?.play_route(!playing);
    setPlaying(!playing);
  };
  const clearRoute = () => {
    getSim()?.clear_route(); // also resets the pick target to start
    setTarget("start");
    setStart(null);
    setEnd(null);
    setRoute(null);
    setPlaying(false);
  };

  return (
    <div className="flex h-dvh flex-col-reverse overflow-hidden bg-bg lg:flex-row">
      <aside
        aria-label="Planner"
        className={`z-10 flex min-h-0 flex-col border-t border-line bg-surface lg:border-r lg:border-t-0 ${open ? "flex-1 lg:w-[380px] lg:flex-none" : "flex-none"}`}
      >
        <header className="flex h-14 flex-none items-center justify-between gap-3 border-b border-line pl-4 pr-2">
          <div className={`min-w-0 ${open ? "" : "lg:hidden"}`}>
            <p className="font-mono text-[10px] uppercase tracking-[0.18em] text-signal">Jezero crater · EVA</p>
            <h1 className="truncate font-display text-[1.45rem] leading-none">C.R.I.S</h1>
          </div>
          <button
            type="button"
            aria-expanded={open}
            aria-controls="planner-body"
            onClick={() => setOpen(!open)}
            className="inline-flex min-h-11 min-w-11 flex-none items-center justify-center gap-1.5 rounded-md px-3 font-mono text-[11px] uppercase tracking-wider text-muted hover:text-ink"
          >
            <svg
              aria-hidden
              viewBox="0 0 12 12"
              className={`size-3 transition-transform ${open ? "lg:rotate-90" : "rotate-180 lg:-rotate-90"}`}
            >
              <path d="M2 4.5 6 8.5l4-4" fill="none" stroke="currentColor" strokeWidth="1.5" />
            </svg>
            {open ? "Hide" : "Show planner"}
          </button>
        </header>

        {/* relative: absolutely positioned sr-only nodes must scroll with the body, not overflow the aside. */}
        <div id="planner-body" hidden={!open} className="relative min-h-0 flex-1 overflow-y-auto overscroll-contain">
          <RouteSection
            start={start}
            end={end}
            target={target}
            onTarget={selectTarget}
            route={route}
            playing={playing}
            ready={ready}
            onTogglePlay={togglePlay}
            onClear={clearRoute}
          />
          <LimitsSection
            params={params}
            onParams={setParams}
            allowedSlope={allowedSlope(params)}
            hazardPending={meta?.hazard_model.type === "none"}
          />
          <LayersSection
            overlay={overlay}
            onChange={selectOverlay}
            meta={meta}
            allowedSlope={allowedSlope(params)}
            maxRough={params.rough_max_m}
            showTraverse={showTraverse}
            onShowTraverse={changeTraverse}
            vscale={vscale}
            onVscale={changeVscale}
          />
          <ConditionsSection />
          <RawImagesSection sol={sol} onSol={setSol} near={near} />
          <SourcesFooter meta={meta} />
        </div>
      </aside>

      {/* Size depends only on breakpoint + panel state, never on the canvas contents. */}
      <main
        aria-label="3D terrain view"
        onFocus={(e) => setKeysActive(e.target.id === "mars-canvas")}
        onBlur={() => setKeysActive(false)}
        className={`relative min-h-0 min-w-0 ${open ? "h-[46dvh] flex-none" : "flex-1"} lg:h-auto lg:flex-1`}
      >
        <MarsSim className="has-[canvas:focus-visible]:outline-2 has-[canvas:focus-visible]:-outline-offset-4 has-[canvas:focus-visible]:outline-signal" />
        <MapOverlay ready={ready} camera={camera} onCamera={selectCamera} target={target} keysActive={keysActive} />
      </main>
    </div>
  );
}
