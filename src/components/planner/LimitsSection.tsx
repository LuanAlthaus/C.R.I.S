"use client";

// Load (body / suit / payload, max slope) and how strongly the route avoids each terrain factor.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import type { ChangeEvent } from "react";
import type { RouteParams } from "@/lib/marsSim";
import { citeCls, inputCls, labelCls, Section } from "./ui";

type Field = { key: keyof RouteParams; label: string; unit: string; min: number; max: number; step: number };

const LOAD_FIELDS: Field[] = [
  { key: "body_kg", label: "Body", unit: "kg", min: 30, max: 200, step: 1 },
  { key: "suit_kg", label: "Suit", unit: "kg", min: 0, max: 200, step: 1 },
  { key: "payload_kg", label: "Payload", unit: "kg", min: 0, max: 500, step: 1 },
  { key: "s_max_base_deg", label: "Max slope", unit: "°", min: 1, max: 45, step: 0.5 },
  { key: "rough_max_m", label: "Max roughness", unit: "m", min: 0.05, max: 2, step: 0.05 },
];

const AVOID: { key: keyof RouteParams; label: string; hint: string }[] = [
  { key: "w_slope", label: "Steep slopes", hint: "HiRISE DTM slope, relative to the allowed max" },
  { key: "w_rough", label: "Rough terrain (DTM)", hint: "Local relief bumps in the elevation model" },
  { key: "w_texture", label: "Rocky texture (optical)", hint: "Speckled, bouldery look in the HiRISE photo" },
  { key: "w_rocks", label: "Mapped rocks", hint: "Talus and exposed blocky bedrock (Mars 2020 photogeologic map)" },
  { key: "w_hazard", label: "Hazard labels", hint: "Team-labelled rough / impassable patches" },
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

export default function LimitsSection({
  params,
  onParams,
  allowedSlope,
  hazardPending,
}: {
  params: RouteParams;
  onParams: (p: RouteParams) => void;
  allowedSlope: number;
  hazardPending: boolean;
}) {
  return (
    <Section n="02" id="sec-limits" title="Load & avoidance">
      <div className="grid grid-cols-2 gap-3">
        {LOAD_FIELDS.map((f) => (
          <NumberField key={f.key} f={f} params={params} onParams={onParams} />
        ))}
      </div>
      <p className="font-mono text-xs tabular-nums">
        <span className="text-muted">Allowed slope with this load:</span> {allowedSlope.toFixed(1)}°
      </p>

      <fieldset className="space-y-3 rounded-lg border border-line p-3">
        <legend className="px-1 text-sm font-medium">What the route avoids</legend>
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
                className="h-11 w-full cursor-pointer accent-signal"
              />
              <p id={`${id}-hint`} className="-mt-2 text-xs text-muted">
                {a.hint}
              </p>
            </div>
          );
        })}
      </fieldset>

      <div className={`${citeCls} space-y-1`}>
        <p>Suit: 104 kg = NASA Mars suit mass target (NTRS 20260005410).</p>
        <p>
          Max slope 20° = NASA suited-walking limit for Artemis (EVA-EXP-0042 Rev B); NASA has no Mars value yet.
          Max roughness 0.30 m = suited step height NASA tested (ICES-2025-294), under Perseverance&apos;s 40 cm
          obstacle limit. Walking speed (1.0 m/s on flat ground) is a team estimate. Mars gravity 3.71 m/s².
        </p>
      </div>
    </Section>
  );
}
