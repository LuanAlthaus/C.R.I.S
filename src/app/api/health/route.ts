import { isSupabaseConfigured } from "@/lib/supabase/server";

// GET /api/health — verifica rápido qué está configurado en el deploy.
export async function GET() {
  return Response.json({
    ok: true,
    supabase: isSupabaseConfigured() ? "configured" : "missing env vars",
    nasaApiKey: process.env.NASA_API_KEY ? "custom" : "DEMO_KEY (30 req/h por IP)",
    time: new Date().toISOString(),
  });
}
