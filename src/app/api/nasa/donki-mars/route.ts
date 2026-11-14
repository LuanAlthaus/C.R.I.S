import { cacheLife } from "next/cache";

// GET /api/nasa/donki-mars?date=YYYY-MM-DD
// CME arrivals at Mars predicted by DONKI WSA-Enlil runs in the 7 days before `date`.
// `alert` = an arrival falls within [date-1d, date+3d] (EVA planning window).
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.

const DAY_MS = 86_400_000;

interface Impact {
  location: string;
  arrivalTime: string;
  isGlancingBlow: boolean;
}
interface EnlilSim {
  simulationID: string;
  impactList: Impact[] | null;
  cmeInputs?: { cmeid: string }[];
}

async function getEnlil(start: string, end: string): Promise<EnlilSim[]> {
  "use cache";
  cacheLife("hours");

  const url = new URL("https://api.nasa.gov/DONKI/WSAEnlilSimulations");
  url.searchParams.set("startDate", start);
  url.searchParams.set("endDate", end);
  url.searchParams.set("api_key", process.env.NASA_API_KEY || "DEMO_KEY");

  const res = await fetch(url);
  if (!res.ok) throw new Error(`DONKI responded ${res.status}`);
  return res.json();
}

export async function GET(request: Request) {
  const raw = new URL(request.url).searchParams.get("date");
  const date = raw ?? new Date().toISOString().slice(0, 10);
  const t = Date.parse(`${date}T00:00:00Z`);
  if (!/^\d{4}-\d{2}-\d{2}$/.test(date) || Number.isNaN(t)) {
    return Response.json({ error: "invalid date" }, { status: 400 });
  }

  let sims: EnlilSim[];
  try {
    sims = await getEnlil(new Date(t - 7 * DAY_MS).toISOString().slice(0, 10), date);
  } catch (e) {
    return Response.json({ error: (e as Error).message }, { status: 502 });
  }

  const byArrival = new Map<string, { simulationID: string; arrivalTime: string; isGlancingBlow: boolean; cmeIDs: string[] }>();
  for (const sim of sims ?? []) {
    for (const hit of sim.impactList ?? []) {
      if (hit.location !== "Mars" || byArrival.has(hit.arrivalTime)) continue;
      byArrival.set(hit.arrivalTime, {
        simulationID: sim.simulationID,
        arrivalTime: hit.arrivalTime,
        isGlancingBlow: hit.isGlancingBlow,
        cmeIDs: (sim.cmeInputs ?? []).map((c) => c.cmeid),
      });
    }
  }
  const events = [...byArrival.values()].sort((a, b) => a.arrivalTime.localeCompare(b.arrivalTime));
  const alert = events.some((e) => {
    const a = Date.parse(e.arrivalTime.endsWith("Z") ? e.arrivalTime : `${e.arrivalTime}Z`);
    return a >= t - DAY_MS && a <= t + 3 * DAY_MS;
  });
  return Response.json({ date, events, alert });
}
