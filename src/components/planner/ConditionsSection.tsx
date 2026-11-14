"use client";

// EVA conditions: live CME arrivals at Mars (DONKI) plus static Mars-vs-Earth cards from conditions.json.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.
import { useEffect, useState, useSyncExternalStore } from "react";
import { NASA_LIMITS } from "@/lib/marsSim";
import { citeCls, inputCls, labelCls, Section } from "./ui";

interface Donki {
  date: string;
  events: { simulationID: string; arrivalTime: string; isGlancingBlow: boolean; cmeIDs: string[] }[];
  alert: boolean;
}

interface Conditions {
  mars: {
    source: string;
    sols: { sol: number; min_c: number; max_c: number; swing_c: number; pressure_pa: number; first_utc: string }[];
    mean_swing_c: number;
    max_swing_c: number;
    mean_pressure_pa: number;
  };
  earth: { source: string; name: string; days: { date: string }[]; mean_swing_c: number };
  earth_pressure_pa: number;
}

const noSubscribe = () => () => {};
const todayUtc = () => new Date().toISOString().slice(0, 10);

const cardCls = "rounded-lg border border-line bg-bg p-3";
const titleCls = "text-sm font-medium";

function Bar({ label, value, max, text, tone }: { label: string; value: number; max: number; text: string; tone: string }) {
  return (
    <div>
      <p className="flex justify-between gap-3 text-sm">
        <span className="text-muted">{label}</span>
        <span className="font-mono tabular-nums">{text}</span>
      </p>
      <div aria-hidden className="mt-1 h-1.5 overflow-hidden rounded-full bg-line">
        <div className={`h-full min-w-0.5 rounded-full ${tone}`} style={{ width: `${(value / max) * 100}%` }} />
      </div>
    </div>
  );
}

// Temperature strip from -110 °C to +60 °C: Mars air range vs. the bare-skin contact limits.
const T_MIN = -110;
const T_SPAN = 170;
const tPos = (c: number) => `${((c - T_MIN) / T_SPAN) * 100}%`;

function SkinScale({ min, max }: { min: number; max: number }) {
  const s = NASA_LIMITS.skin_contact;
  return (
    <div
      role="img"
      aria-label={`Mars air ${min.toFixed(0)} to ${max.toFixed(0)} °C; bare skin safe for unlimited contact only from ${s.unlimited_min_c} to ${s.unlimited_max_c} °C, tissue damage at or below ${s.tissue_damage_cold_c} °C or at or above ${s.tissue_damage_hot_c} °C.`}
      className="relative mt-2 h-14"
    >
      <div className="absolute inset-x-0 top-5 h-2 rounded-full bg-line" />
      <div
        className="absolute top-5 h-2 rounded-full bg-[#3b6fd8]"
        style={{ left: tPos(min), width: `${((max - min) / T_SPAN) * 100}%` }}
      />
      <div
        className="absolute top-5 h-2 rounded-full bg-ok"
        style={{ left: tPos(s.unlimited_min_c), width: `${((s.unlimited_max_c - s.unlimited_min_c) / T_SPAN) * 100}%` }}
      />
      {[s.tissue_damage_cold_c, s.tissue_damage_hot_c].map((c) => (
        <div key={c} className="absolute top-3.5 h-5 w-0.5 bg-danger" style={{ left: tPos(c) }} />
      ))}
      <span className="absolute top-0 -translate-x-1/2 font-mono text-[10px] text-ink" style={{ left: tPos((min + max) / 2) }}>
        Mars air
      </span>
      <span className="absolute top-0 -translate-x-1/2 font-mono text-[10px] text-ok" style={{ left: tPos(29) }}>
        Skin OK
      </span>
      {[-100, -50, 0, 50].map((c) => (
        <span
          key={c}
          className="absolute top-9 -translate-x-1/2 whitespace-nowrap font-mono text-[10px] tabular-nums text-muted"
          style={{ left: tPos(c) }}
        >
          {c > 0 ? `+${c}` : c} °C
        </span>
      ))}
    </div>
  );
}

export default function ConditionsSection() {
  const today = useSyncExternalStore(noSubscribe, todayUtc, () => "");
  const [picked, setPicked] = useState<string | null>(null);
  const date = picked ?? today;
  const [donki, setDonki] = useState<{ date: string; data?: Donki; error?: string } | null>(null);
  const [cond, setCond] = useState<Conditions | "missing" | null>(null);

  useEffect(() => {
    fetch("/data/jezero/conditions.json")
      .then((r) => (r.ok ? r.json() : "missing"))
      .then(setCond)
      .catch(() => setCond("missing"));
  }, []);

  useEffect(() => {
    if (!date) return;
    const ac = new AbortController();
    fetch(`/api/nasa/donki-mars?date=${date}`, { signal: ac.signal })
      .then(async (r) => {
        const j = await r.json();
        setDonki(r.ok ? { date, data: j } : { date, error: r.status === 502 ? "DONKI is not responding right now." : j.error });
      })
      .catch(() => {
        if (!ac.signal.aborted) setDonki({ date, error: "Could not reach the server." });
      });
    return () => ac.abort();
  }, [date]);

  const loading = !!date && donki?.date !== date;
  const c = cond && cond !== "missing" ? cond : null;
  const marsMin = c ? Math.min(...c.mars.sols.map((s) => s.min_c)) : 0;
  const marsMax = c ? Math.max(...c.mars.sols.map((s) => s.max_c)) : 0;
  const swingMax = c ? Math.max(c.mars.max_swing_c, c.earth.mean_swing_c) : 1;
  const emuPa = NASA_LIMITS.suit_pressure_kpa.emu * 1000;
  const rad = NASA_LIMITS.radiation;

  return (
    <Section n="04" id="sec-conditions" title="Conditions">
      <div>
        <label htmlFor="eva-date" className={labelCls}>
          EVA date (UTC)
        </label>
        <input
          id="eva-date"
          type="date"
          value={date}
          onChange={(e) => setPicked(e.target.value)}
          className={`${inputCls} mt-1`}
        />
      </div>

      <div aria-live="polite" className="space-y-2">
        {!date ? (
          <p className="text-sm text-muted">Pick a date to check space weather.</p>
        ) : loading ? (
          <p className="text-sm text-muted">Checking CME arrivals at Mars…</p>
        ) : donki?.error ? (
          <p className="rounded-md bg-danger-soft px-3 py-2 text-sm text-danger">
            Space weather unavailable: {donki.error}
          </p>
        ) : donki?.data ? (
          <>
            {donki.data.alert ? (
              <p className="rounded-md border border-danger bg-danger-soft px-3 py-2.5 text-sm font-medium text-danger">
                CME predicted to reach Mars near this date — postpone EVA / stay near shelter.
              </p>
            ) : (
              <p className="rounded-md border border-line px-3 py-2.5 text-sm">
                No CME predicted to reach Mars between 1 day before and 3 days after this date.
              </p>
            )}
            {donki.data.events.length > 0 && (
              <ul className="divide-y divide-line rounded-md border border-line text-xs">
                {donki.data.events.map((e) => (
                  <li key={e.arrivalTime} className="px-3 py-2">
                    <p className="font-mono tabular-nums">
                      Arrival {e.arrivalTime.replace("T", " ").replace("Z", "")} UTC
                      {e.isGlancingBlow && <span className="text-muted"> · glancing blow</span>}
                    </p>
                    <p className="mt-0.5 break-all text-muted">{e.cmeIDs.join(", ") || e.simulationID}</p>
                  </li>
                ))}
              </ul>
            )}
            <p className={citeCls}>WSA-Enlil runs from the 7 days before this date. NASA CCMC DONKI via api.nasa.gov.</p>
          </>
        ) : null}
      </div>

      {c && (
        <>
          <article className={cardCls} aria-labelledby="card-thermal">
            <h3 id="card-thermal" className={titleCls}>
              Daily temperature swing
            </h3>
            <div className="mt-2.5 space-y-2.5">
              <Bar
                label="Mars air, mean"
                value={c.mars.mean_swing_c}
                max={swingMax}
                text={`${c.mars.mean_swing_c.toFixed(0)} °C`}
                tone="bg-[#d6452f]"
              />
              <Bar
                label="Mars air, max"
                value={c.mars.max_swing_c}
                max={swingMax}
                text={`${c.mars.max_swing_c.toFixed(0)} °C`}
                tone="bg-[#d6452f]"
              />
              <Bar
                label={`${c.earth.name} surface, mean`}
                value={c.earth.mean_swing_c}
                max={swingMax}
                text={`${c.earth.mean_swing_c.toFixed(0)} °C`}
                tone="bg-muted"
              />
            </div>
            <p className={`${citeCls} mt-2`}>
              Mars: {c.mars.source}, {c.mars.sols.length} sols. Measured at Elysium Planitia, not Jezero. Earth:{" "}
              {c.earth.source}, {c.earth.days[0]?.date} to {c.earth.days.at(-1)?.date}.
            </p>

            <h3 className={`${titleCls} mt-4`}>Bare-skin contact</h3>
            <p className="mt-1 text-sm">
              Mars air {marsMin.toFixed(0)} to {marsMax.toFixed(0)} °C vs. safe contact{" "}
              {NASA_LIMITS.skin_contact.unlimited_min_c}–{NASA_LIMITS.skin_contact.unlimited_max_c} °C:{" "}
              <strong className="font-medium text-danger">contact unsafe without suit</strong>.
            </p>
            <SkinScale min={marsMin} max={marsMax} />
            <p className={citeCls}>
              Red ticks: tissue damage at ≤{NASA_LIMITS.skin_contact.tissue_damage_cold_c} °C or ≥
              {NASA_LIMITS.skin_contact.tissue_damage_hot_c} °C. {NASA_LIMITS.skin_contact.cite}.
            </p>
          </article>

          <article className={cardCls} aria-labelledby="card-atmo">
            <h3 id="card-atmo" className={titleCls}>
              Atmospheric pressure
            </h3>
            <div className="mt-2.5 space-y-2.5">
              <Bar
                label="Mars, mean"
                value={c.mars.mean_pressure_pa}
                max={c.earth_pressure_pa}
                text={`${c.mars.mean_pressure_pa.toFixed(0)} Pa`}
                tone="bg-[#d6452f]"
              />
              <Bar label="EMU suit" value={emuPa} max={c.earth_pressure_pa} text={`${emuPa.toLocaleString("en-US")} Pa`} tone="bg-route" />
              <Bar
                label="Earth, sea level"
                value={c.earth_pressure_pa}
                max={c.earth_pressure_pa}
                text={`${c.earth_pressure_pa.toLocaleString("en-US")} Pa`}
                tone="bg-muted"
              />
            </div>
            <p className="mt-2 text-sm">
              Mars air is {((c.mars.mean_pressure_pa / c.earth_pressure_pa) * 100).toFixed(2)}% of Earth&apos;s and{" "}
              {((c.mars.mean_pressure_pa / emuPa) * 100).toFixed(1)}% of suit pressure.
            </p>
            <p className={`${citeCls} mt-1`}>
              Mars: {c.mars.source}. Suit: {NASA_LIMITS.suit_pressure_kpa.cite}.
            </p>
          </article>
        </>
      )}

      <article className={cardCls} aria-labelledby="card-crew">
        <h3 id="card-crew" className={titleCls}>
          Crew limits
        </h3>
        <dl className="mt-2 space-y-2 text-sm">
          {[
            ["Career radiation exposure", `${rad.career_msv} mSv`, rad.career_cite],
            ["Solar particle event design", `${rad.spe_design_msv} mSv`, rad.spe_cite],
            ["EVA hydration", `≥ ${NASA_LIMITS.hydration_ml_per_h.eva_min} mL/h`, NASA_LIMITS.hydration_ml_per_h.cite],
          ].map(([term, value, cite]) => (
            <div key={term} className="flex flex-wrap justify-between gap-x-3">
              <dt className="text-muted">{term}</dt>
              <dd className="font-mono tabular-nums">{value}</dd>
              <dd className={`${citeCls} basis-full`}>{cite}</dd>
            </div>
          ))}
        </dl>
        <p className="mt-2.5 border-t border-line pt-2 text-xs leading-relaxed text-muted">
          The Sky exposure layer is a relative terrain-shielding proxy from the DTM, not a dose. Use it to prefer
          sheltered cells, not to estimate mSv.
        </p>
      </article>
    </Section>
  );
}
