-- Esquema de ejemplo para el starter. Ejecutar con `supabase db push`
-- o pegándolo en el SQL Editor del dashboard de Supabase.

create table if not exists public.team_notes (
  id bigint generated always as identity primary key,
  created_at timestamptz not null default now(),
  challenge_slug text,
  content text not null check (char_length(content) between 1 and 2000)
);

-- RLS SIEMPRE activado. Sin políticas, nadie puede leer ni escribir con la publishable key.
alter table public.team_notes enable row level security;

-- Políticas abiertas para la demo del hackathon (lectura e inserción anónimas).
-- Antes de usar datos reales, restringe a usuarios autenticados: `to authenticated`
-- y una columna user_id = auth.uid().
create policy "demo: anyone can read notes"
  on public.team_notes for select
  to anon, authenticated
  using (true);

create policy "demo: anyone can insert notes"
  on public.team_notes for insert
  to anon, authenticated
  with check (true);

-- Patrón útil para datos procesados (p. ej. focos de calor, series temporales):
-- create table public.observations (
--   id bigint generated always as identity primary key,
--   source text not null,            -- 'VIIRS_SNPP', 'MODIS', 'NISAR', ...
--   observed_at timestamptz not null,
--   lat double precision not null,
--   lon double precision not null,
--   value double precision,
--   props jsonb default '{}'::jsonb
-- );
-- create index on public.observations (observed_at);
-- Para consultas espaciales: `create extension if not exists postgis;` y una columna geography(Point, 4326).
