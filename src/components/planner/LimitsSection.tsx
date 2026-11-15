"use client";

// Two panel rows: "Crew and suit" (who travels, its load and hard limits) and "Avoidance" (how strongly
// the route avoids each terrain factor).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { ChangeEvent } from "react";
import type { RouteParams, Vehicle } from "@/lib/marsSim";
import { CrewIcon, ShieldIcon } from "./icons";
import { citeCls, Collapsible, inputCls, labelCls } from "./ui";

type NumKey = Exclude<keyof RouteParams, "vehicle">;
type Field = { key: NumKey; label: string; unit: string; min: number; max: number; step: number };

const ROUGH: Field = { key: "rough_max_m", label: "Max roughness", unit: "m", min: 0.05, max: 2, step: 0.05 };

const LOAD_FIELDS: Record<Vehicle, Field[]> = {
  eva: [
    { key: "body_kg", label: "Body", unit: "kg", min: 30, max: 200, step: 1 },
    { key: "suit_kg", label: "Suit", unit: "kg", min: 0, max: 200, step: 1 },
    { key: "payload_kg", label: "Payload", unit: "kg", min: 0, max: 500, step: 1 },
    { key: "s_max_base_deg", label: "Max slope", unit: "°", min: 1, max: 45, step: 0.5 },
    ROUGH,
  ],
  rover: [
    { key: "rover_payload_kg", label: "Payload", unit: "kg", min: 0, max: 2000, step: 1 },
    { key: "rover_rated_payload_kg", label: "Rated payload", unit: "kg", min: 0, max: 2000, step: 1 },
    { key: "rover_s_max_deg", label: "Max slope", unit: "°", min: 1, max: 60, step: 0.5 },
    ROUGH,
  ],
};

const VEHICLES: { id: Vehicle; label: string }[] = [
  { id: "eva", label: "Astronaut (EVA)" },
  { id: "rover", label: "Rover" },
];

const AVOID: { key: NumKey; label: string; hint: string }[] = [
  { key: "w_slope", label: "Steep slopes", hint: "HiRISE DTM slope, relative to the allowed max" },
  { key: "w_rough", label: "Rough terrain (DTM)", hint: "Local relief bumps in the elevation model" },
  { key: "w_texture", label: "Rocky texture (optical)", hint: "Speckled, bouldery look in the HiRISE photo" },
  { key: "w_rocks", label: "Mapped rocks", hint: "Talus and exposed blocky bedrock (Mars 2020 photogeologic map)" },
  { key: "w_hazard", label: "Hazard classifier", hint: "Rough / impassable patches predicted from HiRISE" },
  { key: "w_thermal", label: "Thermal swing", hint: "THEMIS relative day–night swing" },
  { key: "w_rad", label: "Sky exposure", hint: "Open sky = less terrain shielding from radiation" },
];

function NumberField({ f, params, onParams }: { f: Field; params: RouteParams; onParams: (p: RouteParams) => void }) {
  const id = `param-${f.key}`;
  // Uncontrolled so the user can clear and retype; only valid numbers reach the sim.
  const onChange = (e: ChangeEvent<HTMLInputElement>) => {
    const v = e.currentTarget.valueAsNumber;
    if (e.currentTarget.validity.valid && Number.isFinite(v)) onParams({ ...params, [f.key]: v });
  };
  return (
    <div>
      <label htmlFor={id} className={labelCls}>
        {f.label} <span className="normal-case tracking-normal">({f.unit})</span>
      </label>
      <input
        id={id}
        type="number"
        inputMode="decimal"
        required
        min={f.min}
        max={f.max}
        step={f.step}
        defaultValue={params[f.key]}
        onChange={onChange}
        className={`${inputCls} mt-1`}
      />
    </div>
  );
}

export function CrewSection({
  params,
  onParams,
  allowedSlope,
}: {
  params: RouteParams;
  onParams: (p: RouteParams) => void;
  allowedSlope: number;
}) {
  const rover = params.vehicle === "rover";
  const load = rover ? params.rover_payload_kg : params.body_kg + params.suit_kg + params.payload_kg;
  return (
    <Collapsible
      id="sec-crew"
      icon={<CrewIcon />}
      title={rover ? "Rover and load" : "Crew and suit"}
      summary={`${rover ? "Rover" : "EVA"} · ${load} kg · ≤${allowedSlope.toFixed(1)}°`}
    >
      <fieldset>
        <legend className={labelCls}>Who travels the route</legend>
        <div className="mt-1.5 grid grid-cols-2 gap-2">
          {VEHICLES.map((v) => (
            <label
              key={v.id}
              className={`flex min-h-10 cursor-pointer items-center justify-center rounded-lg border px-3 text-[13px] transition-colors has-focus-visible:outline-2 has-focus-visible:outline-signal ${params.vehicle === v.id ? "border-signal/60 bg-signal-soft font-medium text-ink" : "border-line text-muted hover:border-white/25 hover:text-ink"}`}
            >
              <input
                type="radio"
                name="vehicle"
                value={v.id}
                checked={params.vehicle === v.id}
                onChange={() => onParams({ ...params, vehicle: v.id })}
                className="sr-only"
              />
              {v.label}
            </label>
          ))}
        </div>
      </fieldset>
      <div className="grid grid-cols-2 gap-3">
        {LOAD_FIELDS[params.vehicle].map((f) => (
          <NumberField key={f.key} f={f} params={params} onParams={onParams} />
        ))}
      </div>
      <p className="font-mono text-xs tabular-nums">
        <span className="text-muted">Allowed slope with this load:</span> {allowedSlope.toFixed(1)}°
      </p>
      {rover && (
        <p className="rounded-lg border border-line bg-bg p-2.5 text-xs leading-relaxed">
          <strong className="font-medium">Experimental setting.</strong> The rover&apos;s {params.rover_s_max_deg}°
          limit is a simulation parameter, not a slope Perseverance has been shown to climb. Rover mode uses its own
          hazard layer without the 20° walking rule; mapped ripples and other flagged ground still block it.
        </p>
      )}
      <LimitNotes rover={rover} />
    </Collapsible>
  );
}

export function AvoidanceSection({
  params,
  onParams,
  hazardPending,
}: {
  params: RouteParams;
  onParams: (p: RouteParams) => void;
  hazardPending: boolean;
}) {
  const on = AVOID.filter((a) => params[a.key] > 0).length;
  return (
    <Collapsible id="sec-avoid" icon={<ShieldIcon />} title="Avoidance" summary={`${on} of ${AVOID.length} active`}>
      <fieldset className="space-y-3">
        <legend className="sr-only">What the route avoids</legend>
        <p className={citeCls}>0 = ignore. Higher = the route detours further around it.</p>
        {AVOID.map((a) => {
          const id = `param-${a.key}`;
          return (
            <div key={a.key}>
              <div className="flex items-baseline justify-between gap-3">
                <label htmlFor={id} className="text-sm">
                  {a.label}
                  {a.key === "w_hazard" && hazardPending && (
                    <span className="ml-1.5 font-mono text-[10px] uppercase tracking-wider text-muted">
                      not computed yet
                    </span>
                  )}
                </label>
                <output htmlFor={id} className="font-mono text-xs tabular-nums">
                  ×{params[a.key].toFixed(1)}
                </output>
              </div>
              <input
                id={id}
                type="range"
                min={0}
                max={10}
                step={0.5}
                value={params[a.key]}
                onChange={(e) => onParams({ ...params, [a.key]: e.currentTarget.valueAsNumber })}
                aria-describedby={`${id}-hint`}
                className="h-9 w-full cursor-pointer accent-signal"
              />
              <p id={`${id}-hint`} className="-mt-1.5 text-xs text-muted">
                {a.hint}
              </p>
            </div>
          );
        })}
      </fieldset>
    </Collapsible>
  );
}

function LimitNotes({ rover }: { rover: boolean }) {
  return (
    <div className={`${citeCls} space-y-1`}>
      {rover ? (
        <p>
          Rover mass 1,025 kg and top speed 4.2 cm/s on flat hard ground = Perseverance (NASA Mars 2020 rover facts).
          Max slope 45° and rated payload 100 kg are team simulation parameters; loads above the rated payload lower
          the slope limit in proportion to total mass. Max roughness 0.30 m is kept from the EVA settings, under
          Perseverance&apos;s 40 cm obstacle limit.
        </p>
      ) : (
        <>
          <p>Suit: 104 kg = NASA Mars suit mass target (NTRS 20260005410).</p>
          <p>
            Max slope 20° = NASA suited-walking limit for Artemis (EVA-EXP-0042 Rev B); NASA has no Mars value yet.
            Max roughness 0.30 m = suited step height NASA tested (ICES-2025-294), under Perseverance&apos;s 40 cm
            obstacle limit. Walking speed (1.0 m/s on flat ground) is a team estimate. Mars gravity 3.71 m/s².
          </p>
        </>
      )}
    </div>
  );
}
