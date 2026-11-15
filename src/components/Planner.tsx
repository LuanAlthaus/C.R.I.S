"use client";

// C.R.I.S Map Viewer shell: full-bleed 3D sim with the planner as a floating dark panel on the left
// (desktop) or a card under the map (mobile). Holds the UI state the sim needs and replays it on ready.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useState } from "react";
import MarsSim from "@/components/MarsSim";
import ConditionsSection from "@/components/planner/ConditionsSection";
import LayersSection from "@/components/planner/LayersSection";
import { PanelIcon, PauseIcon, PlayIcon } from "@/components/planner/icons";
import { AvoidanceSection, CrewSection } from "@/components/planner/LimitsSection";
import MapOverlay from "@/components/planner/MapOverlay";
import RawImagesSection, { type NearestStop } from "@/components/planner/RawImagesSection";
import RouteSection from "@/components/planner/RouteSection";
import SourcesFooter from "@/components/planner/SourcesFooter";
import { buttonCls, labelCls, primaryCls, type SiteMeta } from "@/components/planner/ui";
import {
  allowedSlope,
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

const ROVER_SPEEDS = [1, 2, 4] as const;
type RoverSpeed = (typeof ROVER_SPEEDS)[number];

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
  const [roverSpeed, setRoverSpeed] = useState<RoverSpeed>(1);

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
        sim?.set_rover_speed?.(roverSpeed);
      }),
    [camera, overlay, params, vscale, showTraverse, roverSpeed],
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
    // The sim makes the point the rover just reached its new start and clears the end: the rover
    // stays there and the next click picks where it goes from here.
    const offStopped = onSimEvent<PickEvent>("mars-rover-stopped", (p) => {
      setPlaying(false);
      setStart(p);
      setEnd(null);
      setRoute(null);
    });
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
  const changeRoverSpeed = (s: RoverSpeed) => {
    setRoverSpeed(s);
    getSim()?.set_rover_speed?.(s);
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

  const ok = route?.status === "ok";

  return (
    // theme-hud: the viewer is always the dark mission panel, whatever the colour scheme.
    // Mobile: map on top, panel card under it. Desktop: full-bleed map, panel floating on the left.
    <div className="theme-hud relative flex h-dvh flex-col-reverse overflow-hidden bg-bg text-ink lg:block">
      <aside
        aria-label="Planner"
        className={`relative z-20 m-2 mt-0 flex min-h-0 flex-col overflow-hidden rounded-2xl border border-white/[0.07] bg-surface/95 shadow-[0_30px_80px_-20px_rgba(0,0,0,0.85)] backdrop-blur-xl lg:absolute lg:left-4 lg:top-4 lg:m-0 lg:w-[360px] ${open ? "flex-1 lg:bottom-4" : "flex-none"}`}
      >
        <header className="flex flex-none items-center justify-between gap-3 px-4 pb-3 pt-3.5">
          <div className="min-w-0">
            <h1 className="truncate text-[19px] font-semibold leading-tight tracking-[0.16em]">C.R.I.S.</h1>
            <p className="mt-0.5 flex items-center gap-1.5 text-[12px] text-muted">
              <span aria-hidden className="size-1.5 rounded-full bg-gold shadow-[0_0_8px_var(--gold)]" />
              Jezero crater, Mars
            </p>
          </div>
          <button
            type="button"
            aria-expanded={open}
            aria-controls="planner-body"
            aria-label={open ? "Collapse planner panel" : "Expand planner panel"}
            title={open ? "Collapse" : "Expand"}
            onClick={() => setOpen(!open)}
            className="grid size-9 flex-none place-items-center rounded-lg border border-line text-muted transition-colors hover:bg-white/[0.06] hover:text-ink"
          >
            <PanelIcon className={`size-4 transition-transform ${open ? "" : "rotate-180"}`} />
          </button>
        </header>

        {/* relative: absolutely positioned sr-only nodes must scroll with the body, not overflow the aside. */}
        <div
          id="planner-body"
          hidden={!open}
          className="relative min-h-0 flex-1 overflow-y-auto overscroll-contain [scrollbar-color:rgb(255_255_255/0.15)_transparent] [scrollbar-width:thin]"
        >
          <RouteSection start={start} end={end} target={target} onTarget={selectTarget} route={route} />
          <CrewSection params={params} onParams={setParams} allowedSlope={allowedSlope(params)} />
          <AvoidanceSection
            params={params}
            onParams={setParams}
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
          <p className="border-t border-line px-4 py-3 text-[11px] text-muted">Data: NASA. Not endorsed by NASA.</p>
        </div>

        <footer hidden={!open} className="flex-none space-y-2.5 border-t border-line p-3">
          <fieldset className="flex items-center justify-between gap-3 px-1">
            <legend className="sr-only">Rover playback speed</legend>
            <span aria-hidden className={labelCls}>
              Rover speed
            </span>
            <div className="flex gap-1 rounded-full border border-line bg-bg p-0.5">
              {ROVER_SPEEDS.map((s) => (
                <label key={s} className="cursor-pointer">
                  <input
                    type="radio"
                    name="rover-speed"
                    value={s}
                    checked={roverSpeed === s}
                    onChange={() => changeRoverSpeed(s)}
                    className="peer sr-only"
                  />
                  <span className="flex min-h-8 min-w-11 items-center justify-center rounded-full px-3 font-mono text-[12px] tabular-nums text-muted transition-colors hover:text-ink peer-checked:bg-signal peer-checked:font-semibold peer-checked:text-gold-ink peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-signal">
                    ×{s}
                  </span>
                </label>
              ))}
            </div>
          </fieldset>
          <div className="grid grid-cols-[1fr_auto] gap-2">
            <button type="button" onClick={togglePlay} disabled={!ok || !ready} className={`${primaryCls} min-h-12`}>
              {playing ? <PauseIcon className="size-3.5" /> : <PlayIcon className="size-3.5" />}
              {playing ? "Pause rover" : "Play rover"}
            </button>
            <button
              type="button"
              onClick={clearRoute}
              disabled={!start && !end && !route}
              className={`${buttonCls} min-h-12 px-5`}
            >
              Clear
            </button>
          </div>
        </footer>
      </aside>

      {/* Size depends only on breakpoint + panel state, never on the canvas contents. */}
      <main
        aria-label="3D terrain view"
        onFocus={(e) => setKeysActive(e.target.id === "mars-canvas")}
        onBlur={() => setKeysActive(false)}
        className={`relative min-h-0 min-w-0 ${open ? "h-[46dvh] flex-none" : "flex-1"} lg:absolute lg:inset-0 lg:h-auto`}
      >
        <MarsSim className="bg-[#0b0908] has-[canvas:focus-visible]:outline-2 has-[canvas:focus-visible]:-outline-offset-4 has-[canvas:focus-visible]:outline-signal" />
        <MapOverlay
          ready={ready}
          camera={camera}
          onCamera={selectCamera}
          target={target}
          keysActive={keysActive}
          route={route}
          panelOpen={open}
        />
      </main>
    </div>
  );
}
