import { createClient, isSupabaseConfigured } from "@/lib/supabase/server";

// Ejemplo mínimo de lectura/escritura en Supabase.
// Tabla creada por supabase/migrations/0001_init.sql (team_notes, con RLS).

export async function GET() {
  if (!isSupabaseConfigured()) {
    return Response.json({ error: "Supabase no configurado" }, { status: 503 });
  }
  const supabase = await createClient();
  const { data, error } = await supabase
    .from("team_notes")
    .select("id, created_at, challenge_slug, content")
    .order("created_at", { ascending: false })
    .limit(50);
  if (error) return Response.json({ error: error.message }, { status: 500 });
  return Response.json(data);
}

export async function POST(request: Request) {
  if (!isSupabaseConfigured()) {
    return Response.json({ error: "Supabase no configurado" }, { status: 503 });
  }
  const body = await request.json().catch(() => null);
  const content = typeof body?.content === "string" ? body.content.trim() : "";
  const challenge_slug = typeof body?.challenge_slug === "string" ? body.challenge_slug : null;
  if (!content || content.length > 2000) {
    return Response.json({ error: "content requerido (máx. 2000 caracteres)" }, { status: 400 });
  }
  const supabase = await createClient();
  const { data, error } = await supabase
    .from("team_notes")
    .insert({ content, challenge_slug })
    .select()
    .single();
  if (error) return Response.json({ error: error.message }, { status: 500 });
  return Response.json(data, { status: 201 });
}
