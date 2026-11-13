import { connection } from "next/server";
import { createClient, isSupabaseConfigured } from "@/lib/supabase/server";

// GET /api/cron/keepalive — lo llama el cron de vercel.json una vez por día.
// Los proyectos Free de Supabase se pausan tras 7 días sin actividad, y el jurado
// evalúa entre diciembre y enero: este ping mantiene activo el demo.
// Si defines CRON_SECRET en Vercel, Vercel envía "Authorization: Bearer <CRON_SECRET>".
// Con cacheComponents no existe `export const dynamic`: `await connection()` obliga a ejecutar
// en cada request (si no, el build la prerenderiza y el ping nunca llega a Supabase).

export async function GET(request: Request) {
  await connection();
  const secret = process.env.CRON_SECRET;
  if (secret && request.headers.get("authorization") !== `Bearer ${secret}`) {
    return new Response("Unauthorized", { status: 401 });
  }
  if (!isSupabaseConfigured()) {
    return Response.json({ ok: false, reason: "Supabase no configurado" });
  }
  const supabase = await createClient();
  const { error } = await supabase.from("team_notes").select("id", { head: true, count: "exact" });
  return Response.json({ ok: !error, error: error?.message ?? null, at: new Date().toISOString() });
}
