"use client";

// Info button floating over the END pin: why the planned route is the safest one, compared with the
// shortest path the same hard limits allow and with the straight line.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useRef, useState } from "react";
import { onSimEvent, type PinsEvent, type RouteResult } from "@/lib/marsSim";
import { chip, fmtLength } from "./ui";

const pct = (v: number) => `${(v * 100).toFixed(0)}%`;

function Why({ r }: { r: RouteResult }) {
  const b = r.baseline;
  const d = r.direct;
  const better = b
    ? [
        [r.max_slope_deg < b.max_slope_deg - 0.05, "max slope"],
        [r.max_rough_m < b.max_rough_m - 0.005, "roughness"],
        [r.mean_hazard < b.mean_hazard - 0.005, "hazard"],
        [r.rock_cells < b.rock_cells, "rock exposure"],
      ]
        .filter(([lower]) => lower)
        .map(([, name]) => name)
    : [];
  const extra = b ? r.length_m - b.length_m : 0;
  const reasons = d
    ? (
        [
          [d.slope, "too steep"],
          [d.rough, "too rough"],
          [d.hazard, "flagged by the hazard model"],
        ] as const
      )
        .filter(([n]) => n > 0)
        .map(([n, label]) => `${n} ${label}`)
        .join(", ")
    : "";
  const td = "py-0.5 pl-2 text-right font-mono tabular-nums";
  return (
    <>
      <h3 id="route-why-title" className="mb-1.5 text-sm font-semibold">
        Why this is the safest route
      </h3>
      <p>Every step stays inside the walking limits:</p>
      <dl className="my-1.5 grid grid-cols-[auto_auto] gap-x-3 gap-y-0.5 font-mono text-[11px] tabular-nums">
        <dt className="text-scrim-muted">Max slope</dt>
        <dd>
          {r.max_slope_deg.toFixed(1)}° &lt; {r.s_max_eff_deg.toFixed(1)}°
        </dd>
        <dt className="text-scrim-muted">Max roughness</dt>
        <dd>
          {r.max_rough_m.toFixed(2)} m ≤ {r.rough_max_m.toFixed(2)} m
        </dd>
        <dt className="text-scrim-muted">Hazard</dt>
        <dd>
          max {pct(r.max_hazard)}, mean {pct(r.mean_hazard)}
        </dd>
        <dt className="text-scrim-muted">Mapped-rock cells</dt>
        <dd>{r.rock_cells}</dd>
      </dl>
      {b && (
        <>
          <table className="my-1.5 w-full text-[11px]">
            <thead className="text-scrim-muted">
              <tr>
                <th scope="col" className="sr-only">
                  Metric
                </th>
                <th scope="col" className="pl-2 text-right font-normal">
                  This route
                </th>
                <th scope="col" className="pl-2 text-right font-normal">
                  Shortest allowed path
                </th>
              </tr>
            </thead>
            <tbody>
              {(
                [
                  ["Length", fmtLength(r.length_m), fmtLength(b.length_m)],
                  ["Max slope", `${r.max_slope_deg.toFixed(1)}°`, `${b.max_slope_deg.toFixed(1)}°`],
                  ["Max roughness", `${r.max_rough_m.toFixed(2)} m`, `${b.max_rough_m.toFixed(2)} m`],
                  ["Mean hazard", pct(r.mean_hazard), pct(b.mean_hazard)],
                  ["Rock cells", String(r.rock_cells), String(b.rock_cells)],
                ] as const
              ).map(([name, ours, theirs]) => (
                <tr key={name}>
                  <th scope="row" className="py-0.5 text-left font-normal text-scrim-muted">
                    {name}
                  </th>
                  <td className={td}>{ours}</td>
                  <td className={td}>{theirs}</td>
                </tr>
              ))}
            </tbody>
          </table>
          <p>
            {extra < 1 && better.length === 0
              ? "The shortest allowed path is already the lowest-risk one."
              : `+${fmtLength(Math.max(extra, 0))} (${((Math.max(extra, 0) / b.length_m) * 100).toFixed(0)}%) longer than the shortest allowed path ` +
                (better.length
                  ? `to lower: ${better.join(", ")}.`
                  : "to keep a lower overall risk score (texture, heat, radiation).")}
          </p>
        </>
      )}
      {d && d.blocked > 0 && (
        <p className="mt-1.5">
          Straight line: crosses {d.blocked} impassable cells ({reasons}).
        </p>
      )}
      <p className="mt-2 text-scrim-muted">
        A* picks the path with the lowest cost = distance × (1 + weighted slope, roughness, rocks, hazard, texture,
        heat and radiation). Hard limits from the Limits panel: slope &lt; {r.s_max_eff_deg.toFixed(1)}°, roughness ≤{" "}
        {r.rough_max_m.toFixed(2)} m.
      </p>
    </>
  );
}

export default function RouteInfoPin({ route }: { route: RouteResult | null }) {
  const wrap = useRef<HTMLDivElement>(null);
  const button = useRef<HTMLButtonElement>(null);
  const dialog = useRef<HTMLDivElement>(null);
  const pinY = useRef(0);
  const [onScreen, setOnScreen] = useState(false);
  const [open, setOpen] = useState(false);
  // Side of the button with more room in the 3D view, and that room in px so the popover never clips.
  const [place, setPlace] = useState({ below: false, room: 0 });

  // Follows the pin every frame without re-rendering: only on/off screen goes through React state.
  useEffect(
    () =>
      onSimEvent<PinsEvent>("mars-pins", ({ end }) => {
        if (end && wrap.current) {
          wrap.current.style.transform = `translate(${end.x}px, ${end.y}px) translate(-50%, -100%)`;
          pinY.current = end.y;
        }
        setOnScreen(end != null);
        if (!end) setOpen(false);
      }),
    [],
  );

  const visible = route?.status === "ok" && onScreen;
  const isOpen = open && visible;

  useEffect(() => {
    if (!isOpen) return;
    dialog.current?.focus();
    const close = () => {
      setOpen(false);
      button.current?.focus();
    };
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && close();
    const onDown = (e: PointerEvent) => {
      if (!wrap.current?.contains(e.target as Node)) close();
    };
    document.addEventListener("keydown", onKey);
    document.addEventListener("pointerdown", onDown);
    return () => {
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("pointerdown", onDown);
    };
  }, [isOpen]);

  return (
    <div ref={wrap} hidden={!visible} className="pointer-events-none absolute left-0 top-0 z-10 pb-1">
      <button
        ref={button}
        type="button"
        aria-label="Why this route is the safest"
        aria-expanded={isOpen}
        aria-controls="route-why"
        onClick={() => {
          const h = wrap.current?.parentElement?.clientHeight ?? 0;
          const [above, below] = [pinY.current - 44 - 16, h - pinY.current - 12];
          setPlace({ below: below > above, room: Math.max(above, below) });
          setOpen((o) => !o);
        }}
        className={`${chip} pointer-events-auto grid size-11 place-items-center transition-colors hover:border-white/40 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-signal`}
      >
        <svg aria-hidden viewBox="0 0 16 16" className="size-4">
          <circle cx="8" cy="8" r="6.5" fill="none" stroke="currentColor" strokeWidth="1.5" />
          <path d="M8 7.2v4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
          <circle cx="8" cy="4.8" r="0.9" fill="currentColor" />
        </svg>
      </button>
      {isOpen && route && (
        <div
          ref={dialog}
          id="route-why"
          role="dialog"
          aria-labelledby="route-why-title"
          tabIndex={-1}
          style={{ maxHeight: place.room }}
          className={`${chip} pointer-events-auto absolute left-1/2 w-[300px] max-w-[calc(100vw-1rem)] -translate-x-1/2 overflow-y-auto p-3 text-xs leading-relaxed outline-none ${place.below ? "top-full mt-2" : "bottom-full mb-2"}`}
        >
          <Why r={route} />
        </div>
      )}
    </div>
  );
}
