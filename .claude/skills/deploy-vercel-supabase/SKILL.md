---
name: deploy-vercel-supabase
description: Desplegar este starter gratis con Vercel (Hobby) + Supabase (Free) — crear el proyecto de Supabase, aplicar migraciones, configurar variables de entorno, conectar el repo de GitHub a Vercel, deploys de preview y de producción, verificación con /api/health y solución de errores comunes. Úsala cuando haya que publicar la app, cambiar variables o diagnosticar un deploy roto. Guía completa paso a paso en docs/deploy/vercel-supabase.md.
---

# Deploy: Vercel + Supabase (gratis)

Guía detallada para humanos: `docs/deploy/vercel-supabase.md`. Límites de los planes gratis: `docs/resources/ofertas-y-free-tiers.md`.
Para dudas específicas de Supabase usa también la skill oficial `supabase` (en `.claude/skills/supabase`).

## Flujo rápido (CLI)
```bash
# 0) Requisitos: cuenta en GitHub, Vercel (login con GitHub) y Supabase
pnpm i -g vercel            # o: npx vercel
npx supabase --version      # CLI de Supabase vía npx (no hace falta instalarla global)

# 1) Supabase
npx supabase login
npx supabase link --project-ref <ref>           # ref = subdominio de https://<ref>.supabase.co
npx supabase db push                            # aplica supabase/migrations/*.sql

# 2) Variables locales
cp .env.example .env.local                      # completar URL + PUBLISHABLE_KEY (+ NASA_API_KEY)
pnpm dev                                        # http://localhost:3000/api/health

# 3) Vercel
vercel link                                     # crea o enlaza el proyecto
vercel env add NEXT_PUBLIC_SUPABASE_URL         # repetir por cada variable (production, preview, development)
vercel env add NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY
vercel env add NASA_API_KEY
vercel                                          # deploy de preview
vercel --prod                                   # producción
```
Lo más simple es importar el repo desde https://vercel.com/new : cada push a `main` despliega a producción y cada PR crea un Preview.

## Reglas
- Las variables `NEXT_PUBLIC_*` se incrustan **al hacer el build**. Si las cambias, vuelve a desplegar.
- **Nunca** pongas `SUPABASE_SECRET_KEY` (ni la antigua `service_role`) con prefijo `NEXT_PUBLIC_` ni en código cliente.
- Cada tabla nueva lleva `enable row level security` y políticas explícitas. Sin políticas, la API devuelve `[]` (no da error): es la causa más común de "no veo datos".
- Datos estáticos pesados: `public/data/` (CDN de Vercel) o Supabase Storage. No en la base de datos.
- Por defecto Vercel corre funciones Node.js. Para procesamiento pesado en Python, pre-procesa offline; los runtimes de Python en Vercel tienen límites de tamaño y duración.

## Verificación después del deploy
1. `curl https://<app>.vercel.app/api/health` → `supabase: "configured"`.
2. `curl https://<app>.vercel.app/api/notes` → `[]` o una lista, no un 500.
3. Abrir en incógnito y en el celular.
4. Revisar logs: `vercel logs <deployment-url>` o el dashboard → Logs.

## Errores comunes
| Síntoma | Causa / solución |
|---|---|
| `supabase: "missing env vars"` en producción | Faltan las variables en Vercel, o se agregaron después del build → redeploy |
| `[]` aunque la tabla tiene filas | RLS sin política `select` para `anon` |
| `new row violates row-level security policy` | Falta la política `insert` |
| El proyecto de Supabase no responde | Los proyectos Free se pausan por inactividad: "Restore" desde el dashboard. Haz un request diario los días previos a la evaluación |
| Build falla en Vercel pero no localmente | Diferencia de versión de Node o un error de tipos/ESLint: corre `pnpm build` local |
| 504 en una API route | Superaste la duración máxima de la función: pre-procesa, cachea o usa streaming |
