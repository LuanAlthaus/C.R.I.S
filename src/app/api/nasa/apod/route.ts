import { cacheLife } from "next/cache";

// GET /api/nasa/apod?date=YYYY-MM-DD
// Ejemplo de proxy a api.nasa.gov: la key queda en el servidor y la respuesta se cachea por horas.
// Usa este patrón para cualquier API con key (Earthdata token, FIRMS MAP_KEY, Meteomatics, LLMs).
// Este proyecto usa `cacheComponents: true` (Next.js 16): el cacheo se hace con 'use cache'
// en una función helper; `export const revalidate`/`dynamic` no están disponibles.

async function getApod(date: string | null) {
  "use cache";
  cacheLife("hours");

  const url = new URL("https://api.nasa.gov/planetary/apod");
  url.searchParams.set("api_key", process.env.NASA_API_KEY || "DEMO_KEY");
  if (date) url.searchParams.set("date", date);

  const res = await fetch(url);
  if (!res.ok) throw new Error(`NASA API respondió ${res.status}`);
  return res.json();
}

export async function GET(request: Request) {
  const raw = new URL(request.url).searchParams.get("date");
  const date = raw && /^\d{4}-\d{2}-\d{2}$/.test(raw) ? raw : null;
  try {
    return Response.json(await getApod(date));
  } catch (e) {
    return Response.json({ error: (e as Error).message }, { status: 502 });
  }
}
