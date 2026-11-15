"use client";

// Route block of the panel: start / end cards, the route metrics grid, the safety badge and a
// discreet disclosure with the elevation profile. Play / Clear live in the panel footer (Planner).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { PickEvent, PickTarget, RouteResult } from "@/lib/marsSim";
import { ChevronIcon, ShieldCheckIcon } from "./icons";
import { fmtEta, fmtLat, fmtLength, fmtLon, labelCls } from "./ui";

const DOT = { start: "var(--start)", end: "var(--end)" } as const;

function EndpointCard({
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
    <div
      className={`relative flex items-center gap-3 rounded-xl border px-3 py-2.5 transition-[border-color,background-color] duration-150 ${active ? "border-signal/50 bg-signal-soft" : "border-line bg-bg"}`}
    >
      <span
        aria-hidden
        className="size-2.5 flex-none rounded-full ring-4 ring-white/[0.04]"
        style={{ background: DOT[which] }}
      />
      <div className="min-w-0 flex-1">
        <p className="flex items-baseline gap-2 text-[13.5px] font-medium">
          {label}
          {active && (
            <span className="font-mono text-[10px] uppercase tracking-wider text-signal">
              {p ? "Click to move" : "Placing"}
            </span>
          )}
        </p>
        {p ? (
          <p className="truncate font-mono text-[11px] tabular-nums text-muted">
            {fmtLat(p.lat)}, {fmtLon(p.lon)}
            <span className="text-muted/70">
              {" "}
              · {p.elevation_m.toFixed(0)} m · {p.slope_deg.toFixed(1)}°
            </span>
          </p>
        ) : (
          <p className="text-[12px] text-muted">{active ? "Click the terrain" : "Not set"}</p>
        )}
      </div>
      <button
        type="button"
        aria-pressed={active}
        aria-label={`${p ? "Change" : "Set"} ${label.toLowerCase()} point`}
        onClick={onSelect}
        className="flex-none rounded-lg px-2.5 py-1.5 text-[12px] font-medium text-signal transition-colors hover:bg-signal-soft aria-pressed:bg-signal-soft"
      >
        {p ? "Change" : "Set"}
      </button>
    </div>
  );
}

function Metric({ term, value, sub, warn }: { term: string; value: string; sub?: string; warn?: boolean }) {
  return (
    <div className="rounded-xl border border-line bg-bg px-3 py-2.5">
      <dt className={labelCls}>{term}</dt>
      <dd className={`mt-1 flex items-baseline gap-1.5 tabular-nums ${warn ? "text-danger" : ""}`}>
        <span className="text-[19px] font-semibold leading-none tracking-tight">{value}</span>
        {sub && <span className="font-mono text-[11px] text-muted">{sub}</span>}
      </dd>
    </div>
  );
}

// Elevation along the route, from mars-route `profile` (absolute metres).
function Profile({ route }: { route: RouteResult }) {
  const pts = route.profile;
  if (pts.length < 2) return null;
  const W = 320;
  const H = 64;
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
        className="h-16 w-full overflow-visible rounded-lg border border-line bg-bg"
      >
        <path d={`${line}L${W},${H}L0,${H}Z`} className="fill-signal/10" />
        <path d={line} fill="none" className="stroke-signal" strokeWidth="1.75" vectorEffect="non-scaling-stroke" />
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

function Row({ term, value }: { term: string; value: string }) {
  return (
    <div className="flex justify-between gap-3 text-[13px]">
      <dt className="text-muted">{term}</dt>
      <dd className="font-mono tabular-nums">{value}</dd>
    </div>
  );
}

export default function RouteSection({
  start,
  end,
  target,
  onTarget,
  route,
}: {
  start: PickEvent | null;
  end: PickEvent | null;
  target: PickTarget;
  onTarget: (t: PickTarget) => void;
  route: RouteResult | null;
}) {
  const ok = route?.status === "ok";
  const step = !start ? 1 : !end ? 2 : 3;
  const blocked = [start, end].filter((p): p is PickEvent => !!p && !p.passable);
  const r = ok ? route : null;
  const rover = (route?.vehicle ?? "eva") === "rover";

  return (
    <section aria-labelledby="sec-route" className="space-y-3 px-4 pb-4">
      <div className="flex items-baseline justify-between">
        <h2 id="sec-route" className="text-[15px] font-semibold tracking-tight">
          Route
        </h2>
        <p className="font-mono text-[10.5px] uppercase tracking-[0.14em] text-muted" aria-live="polite">
          {step === 1 ? "1/3 · place start" : step === 2 ? "2/3 · place end" : "3/3 · review"}
        </p>
      </div>

      <div className="space-y-2" role="group" aria-label="Route points: Change picks which point the next click places">
        <EndpointCard which="start" p={start} active={target === "start"} onSelect={() => onTarget("start")} />
        <EndpointCard which="end" p={end} active={target === "end"} onSelect={() => onTarget("end")} />
      </div>

      {blocked.map((p) => (
        <p key={p.which} role="alert" className="rounded-xl bg-danger-soft px-3 py-2 text-[13px] text-danger">
          The {p.which} point is not walkable with the current limits ({p.slope_deg.toFixed(1)}° slope or a blocked
          cell). Move it, or raise the max slope / lower the load.
        </p>
      ))}

      <div aria-live="polite" className="space-y-2">
        {route && !ok ? (
          <p className="rounded-xl bg-danger-soft px-3 py-2.5 text-[13px] font-medium text-danger">
            No safe path with these limits. Lower the payload, raise the max slope or pick other points.
          </p>
        ) : (
          <>
            <dl className="grid grid-cols-2 gap-2">
              <Metric term="Length" value={r ? fmtLength(r.length_m) : "—"} />
              <Metric
                term={rover ? "Drive time" : "Walking time"}
                value={r ? fmtEta(r.eta_min) : "—"}
                // 210 m/sol: Perseverance's average progress in its fastest campaign (Verma et al. 2023).
                sub={r && rover ? `≈${Math.max(1, Math.ceil(r.length_m / 210))} sols` : undefined}
              />
              <Metric
                term="Max slope"
                value={r ? `${r.max_slope_deg.toFixed(1)}°` : "—"}
                sub={r ? `/ ${r.s_max_eff_deg.toFixed(1)}°` : undefined}
                warn={!!r && r.max_slope_deg > r.s_max_eff_deg}
              />
              <Metric
                term="Climb"
                value={r ? `+${r.ascent_m.toFixed(0)} m` : "—"}
                sub={r ? `−${r.descent_m.toFixed(0)} m` : undefined}
              />
            </dl>
            {r ? (
              <p className="flex items-center gap-2.5 rounded-xl border border-ok/25 bg-ok/10 px-3 py-2.5 text-[13px]">
                <ShieldCheckIcon className="size-4 flex-none text-ok" />
                <span className="min-w-0 flex-1">
                  <span className="font-medium text-ok">Safest route inside limits</span>
                  <span className="block font-mono text-[10.5px] text-muted">
                    slope &lt; {r.s_max_eff_deg.toFixed(1)}° · step ≤ {r.rough_max_m.toFixed(2)} m · hazard{" "}
                    {(r.mean_hazard * 100).toFixed(0)}%
                  </span>
                </span>
              </p>
            ) : (
              <p className="rounded-xl border border-dashed border-line px-3 py-2.5 text-[12.5px] text-muted">
                {step === 1
                  ? "Click the terrain to place the start point."
                  : step === 2
                    ? "Now click where the walk should end."
                    : "Planning the safest route…"}
              </p>
            )}
          </>
        )}
      </div>

      {r && (
        <details className="group/prof rounded-xl border border-line">
          <summary className="flex min-h-10 cursor-pointer list-none items-center justify-between gap-2 px-3 text-[12.5px] text-muted transition-colors hover:text-ink [&::-webkit-details-marker]:hidden">
            Elevation profile &amp; load
            <ChevronIcon className="size-3 transition-transform group-open/prof:rotate-90" />
          </summary>
          <div className="space-y-3 border-t border-line px-3 py-3">
            <Profile route={r} />
            <dl className="space-y-1">
              <Row term="Mean hazard" value={`${(r.mean_hazard * 100).toFixed(0)}%`} />
              <Row term="Weight factor" value={`×${r.weight_factor.toFixed(2)}`} />
            </dl>
            <p className="border-t border-line pt-2 text-[12.5px] text-muted">
              Loaded weight on Mars ={" "}
              <strong className="font-mono font-medium tabular-nums text-ink">
                {(r.weight_factor * 100).toFixed(0)}%
              </strong>{" "}
              of body weight on Earth.
            </p>
          </div>
        </details>
      )}
    </section>
  );
}
