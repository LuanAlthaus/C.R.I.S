"use client";

// Guided walk flow: 1 set start → 2 set end → 3 review (stats + elevation profile) → play the rover.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { PickEvent, PickTarget, RouteResult } from "@/lib/marsSim";
import { buttonCls, fmtEta, fmtLat, fmtLength, fmtLon, labelCls, primaryCls, Section } from "./ui";

const DOT = { start: "#2e9e5b", end: "#d6452f" } as const;

function Stepper({ step }: { step: 1 | 2 | 3 }) {
  return (
    <ol className="grid grid-cols-3 gap-1.5 font-mono text-[11px] uppercase tracking-wider">
      {(["Set start", "Set end", "Review"] as const).map((label, i) => {
        const n = i + 1;
        const state = n < step ? "done" : n === step ? "current" : "todo";
        return (
          <li
            key={label}
            aria-current={state === "current" ? "step" : undefined}
            className={`flex items-center gap-1.5 border-t-2 pt-1.5 ${state === "todo" ? "border-line text-muted" : state === "current" ? "border-signal text-ink" : "border-ok text-muted"}`}
          >
            <span className={state === "current" ? "text-signal" : ""}>{state === "done" ? "✓" : n}</span>
            {label}
          </li>
        );
      })}
    </ol>
  );
}

function EndpointButton({
  which,
  p,
  active,
  onSelect,
}: {
  which: PickTarget;
  p: PickEvent | null;
  active: boolean;
  onSelect: () => void;
}) {
  const label = which === "start" ? "Start" : "End";
  return (
    <button
      type="button"
      aria-pressed={active}
      onClick={onSelect}
      className={`flex min-h-24 w-full flex-col items-start gap-1 rounded-lg border p-3 text-left transition-[border-color,background-color] duration-150 ${active ? "border-signal bg-signal-soft" : "border-line bg-bg hover:border-ink"}`}
    >
      <span className="flex w-full items-center justify-between gap-2">
        <span className="flex items-center gap-2 text-sm font-semibold">
          <span aria-hidden className="size-2.5 rounded-full" style={{ background: DOT[which] }} />
          {label}
        </span>
        {active && <span className="font-mono text-[10px] uppercase tracking-wider text-signal">Placing</span>}
      </span>
      {p ? (
        <span className="font-mono text-[11px] leading-relaxed tabular-nums">
          {fmtLat(p.lat)}
          <br />
          {fmtLon(p.lon)}
          <br />
          <span className="text-muted">
            {p.elevation_m.toFixed(0)} m · {p.slope_deg.toFixed(1)}°
          </span>
        </span>
      ) : (
        <span className="text-xs text-muted">{active ? "Click the terrain" : "Not set"}</span>
      )}
    </button>
  );
}

// Elevation along the route, from mars-route `profile` (absolute metres).
function Profile({ route }: { route: RouteResult }) {
  const pts = route.profile;
  if (pts.length < 2) return null;
  const W = 320;
  const H = 88;
  const PAD = 4;
  const hs = pts.map((q) => q.h);
  const lo = Math.min(...hs);
  const hi = Math.max(...hs);
  const span = Math.max(hi - lo, 1);
  const dMax = pts.at(-1)!.d || 1;
  const xy = pts.map((q) => [(q.d / dMax) * W, PAD + (1 - (q.h - lo) / span) * (H - 2 * PAD)] as const);
  const line = xy.map(([x, y], i) => `${i ? "L" : "M"}${x.toFixed(1)},${y.toFixed(1)}`).join("");
  const [x0, y0] = xy[0];
  const [x1, y1] = xy.at(-1)!;
  return (
    <figure className="space-y-1">
      <figcaption className={`${labelCls} flex justify-between`}>
        <span>Elevation profile</span>
        <span className="normal-case tracking-normal">
          {hi.toFixed(0)} / {lo.toFixed(0)} m
        </span>
      </figcaption>
      <svg
        role="img"
        aria-label={`Elevation profile over ${fmtLength(route.length_m)}: starts at ${hs[0].toFixed(0)} m, ends at ${hs.at(-1)!.toFixed(0)} m, lowest ${lo.toFixed(0)} m, highest ${hi.toFixed(0)} m; ${route.ascent_m.toFixed(0)} m total ascent, ${route.descent_m.toFixed(0)} m total descent.`}
        viewBox={`0 0 ${W} ${H}`}
        preserveAspectRatio="none"
        className="h-22 w-full overflow-visible rounded-md border border-line bg-surface"
      >
        <path d={`${line}L${W},${H}L0,${H}Z`} className="fill-route/15" />
        <path d={line} fill="none" className="stroke-route" strokeWidth="2" vectorEffect="non-scaling-stroke" />
        <circle cx={x0} cy={y0} r="3.5" fill={DOT.start} />
        <circle cx={x1} cy={y1} r="3.5" fill={DOT.end} />
      </svg>
      <p className="flex justify-between font-mono text-[11px] tabular-nums text-muted" aria-hidden>
        <span>0</span>
        <span>{fmtLength(route.length_m)}</span>
      </p>
    </figure>
  );
}

function Stat({ term, value, warn }: { term: string; value: string; warn?: boolean }) {
  return (
    <div className="flex justify-between gap-3 text-sm">
      <dt className="text-muted">{term}</dt>
      <dd className={`font-mono tabular-nums ${warn ? "text-danger" : ""}`}>{value}</dd>
    </div>
  );
}

export default function RouteSection({
  start,
  end,
  target,
  onTarget,
  route,
  playing,
  ready,
  onTogglePlay,
  onClear,
}: {
  start: PickEvent | null;
  end: PickEvent | null;
  target: PickTarget;
  onTarget: (t: PickTarget) => void;
  route: RouteResult | null;
  playing: boolean;
  ready: boolean;
  onTogglePlay: () => void;
  onClear: () => void;
}) {
  const ok = route?.status === "ok";
  const step = !start ? 1 : !end ? 2 : 3;
  const blocked = [start, end].filter((p): p is PickEvent => !!p && !p.passable);

  return (
    <Section n="01" id="sec-route" title="Plan a walk">
      <Stepper step={step} />
      <p className="text-sm leading-relaxed">
        {step === 1
          ? "Click the terrain to place the start point."
          : step === 2
            ? "Now click where the walk should end."
            : "Review the route below. Pick Start or End again to move a point."}
      </p>

      <div className="grid grid-cols-2 gap-2" role="group" aria-label="Which point the next click places">
        <EndpointButton which="start" p={start} active={target === "start"} onSelect={() => onTarget("start")} />
        <EndpointButton which="end" p={end} active={target === "end"} onSelect={() => onTarget("end")} />
      </div>

      {blocked.map((p) => (
        <p key={p.which} role="alert" className="rounded-md bg-danger-soft px-3 py-2 text-sm text-danger">
          The {p.which} point is not walkable with the current limits ({p.slope_deg.toFixed(1)}° slope or a blocked
          cell). Move it, or raise the max slope / lower the load.
        </p>
      ))}

      <div aria-live="polite" className="rounded-lg border border-line bg-bg p-3">
        {!route ? (
          <p className="text-sm text-muted">The safest route appears here once both points are set.</p>
        ) : !ok ? (
          <p className="rounded-md bg-danger-soft px-3 py-2 text-sm font-medium text-danger">
            No safe path with these limits. Lower the payload, raise the max slope or pick other points.
          </p>
        ) : (
          <div className="space-y-3">
            <div className="grid grid-cols-2 gap-3">
              <div>
                <p className={labelCls}>Length</p>
                <p className="font-display text-4xl leading-none tabular-nums">{fmtLength(route.length_m)}</p>
              </div>
              <div>
                <p className={labelCls}>Walking time</p>
                <p className="font-display text-4xl leading-none tabular-nums">{fmtEta(route.eta_min)}</p>
              </div>
            </div>
            <Profile route={route} />
            <dl className="space-y-1">
              <Stat
                term="Max slope / allowed"
                value={`${route.max_slope_deg.toFixed(1)}° / ${route.s_max_eff_deg.toFixed(1)}°`}
                warn={route.max_slope_deg > route.s_max_eff_deg}
              />
              <Stat term="Ascent / descent" value={`+${route.ascent_m.toFixed(0)} m / −${route.descent_m.toFixed(0)} m`} />
              <Stat term="Mean hazard" value={`${(route.mean_hazard * 100).toFixed(0)}%`} />
              <Stat term="Weight factor" value={`×${route.weight_factor.toFixed(2)}`} />
            </dl>
            <p className="border-t border-line pt-2.5 text-sm">
              Loaded weight on Mars ={" "}
              <strong className="font-mono font-medium tabular-nums">{(route.weight_factor * 100).toFixed(0)}%</strong>{" "}
              of body weight on Earth.
            </p>
          </div>
        )}
      </div>

      <div className="grid grid-cols-2 gap-2">
        <button type="button" onClick={onTogglePlay} disabled={!ok || !ready} className={primaryCls}>
          {playing ? "Pause rover" : "Play rover"}
        </button>
        <button type="button" onClick={onClear} disabled={!start && !end && !route} className={buttonCls}>
          Clear
        </button>
      </div>
    </Section>
  );
}
