# Guía de despliegue gratis: Vercel + Supabase

Tiempo estimado: **20–30 minutos** la primera vez. Costo: **US$ 0** (Vercel Hobby + Supabase Free).
Límites de cada plan: [`docs/resources/ofertas-y-free-tiers.md`](../resources/ofertas-y-free-tiers.md).

```
 Navegador ──► Vercel (Next.js: páginas + /api/*) ──► Supabase (Postgres + RLS, Auth, Storage)
                     │                                   ▲
                     ├─► APIs de NASA (key en el servidor) │
                     └─► cron diario /api/cron/keepalive ──┘  (evita la pausa por inactividad)
```

> **Regla del evento:** el código específico del challenge recién se puede empezar el **sábado 14 de noviembre a las 9:00 (hora local)**. Crear cuentas, desplegar este template vacío y preparar herramientas está bien antes.

---

## 0. Cuentas (una sola vez, todas gratis)
1. **GitHub**: https://github.com/signup
2. **Vercel**: https://vercel.com/signup → "Continue with GitHub" → plan **Hobby**.
3. **Supabase**: https://supabase.com/dashboard → login con GitHub.
4. **NASA API key**: https://api.nasa.gov/ (te llega al instante).
5. **Earthdata Login** (si vas a usar datos de la Tierra): https://urs.earthdata.nasa.gov/users/new
6. Requisitos locales: Node.js 20 o superior y `pnpm` (`npm i -g pnpm`), o usa `npm`.

## 1. Crear tu repo desde el template
Opción A, en la web: botón **"Use this template" → "Create a new repository"** en GitHub (público).
Opción B, con la CLI:
```bash
gh repo create mi-equipo-space-apps --public --template marcosferr/nasa-space-apps-starter-template --clone
cd mi-equipo-space-apps
pnpm install
```

## 2. Supabase
1. Dashboard → **New project**. Elige la región más cercana a tus usuarios y guarda la contraseña de la base.
2. Copia los datos con el botón **Connect** (o Project Settings → API Keys):
   - `Project URL` → `NEXT_PUBLIC_SUPABASE_URL`
   - `Publishable key` (`sb_publishable_...`) → `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY`
   - `Secret key` (`sb_secret_...`) → solo para scripts locales de carga de datos. **Nunca va al navegador.**
3. Aplica el esquema de ejemplo. Elige una:
   - **SQL Editor** del dashboard → pegar `supabase/migrations/0001_init.sql` → Run.
   - **CLI:**
     ```bash
     npx supabase login
     npx supabase link --project-ref <ref>     # el <ref> es el subdominio de tu Project URL
     npx supabase db push
     ```
4. Revisa en **Table Editor** que `team_notes` existe y tiene **RLS enabled**.

> Las tablas nuevas siempre llevan `alter table ... enable row level security;` y políticas explícitas. Sin políticas, las consultas devuelven `[]` sin error.

## 3. Correr local
```bash
cp .env.example .env.local      # completa las variables
pnpm dev                        # http://localhost:3000
curl localhost:3000/api/health  # → "supabase": "configured"
curl -X POST localhost:3000/api/notes -H 'content-type: application/json' -d '{"content":"hola"}'
curl localhost:3000/api/notes
```

## 4. Vercel
**Opción A (recomendada): importar desde GitHub**
1. https://vercel.com/new → importar tu repo. Detecta Next.js solo; no cambies los comandos de build.
2. En **Environment Variables**, agrega (para Production, Preview y Development):
   | Variable | Obligatoria |
   |---|---|
   | `NEXT_PUBLIC_SUPABASE_URL` | sí |
   | `NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY` | sí |
   | `NASA_API_KEY` | recomendada (si falta, se usa DEMO_KEY: 30 req/h) |
   | `CRON_SECRET` | recomendada (cadena aleatoria larga: `openssl rand -hex 32`) |
   | `NEXT_PUBLIC_REPO_URL` | opcional (URL de tu repo, para los links de la página de inicio) |
   | `EARTHDATA_TOKEN`, `FIRMS_MAP_KEY`, `GEMINI_API_KEY`… | según tu proyecto |
3. **Deploy**. Desde ahí, cada push a `main` va a producción y cada PR genera una URL de Preview.

**Opción B: CLI**
```bash
npx vercel login
npx vercel link
npx vercel env add NEXT_PUBLIC_SUPABASE_URL          # repite por cada variable
npx vercel env pull .env.local                       # baja las variables a tu máquina
npx vercel                                           # preview
npx vercel --prod                                    # producción
```

> Las variables `NEXT_PUBLIC_*` se incrustan al hacer el build. Si las cambias, **vuelve a desplegar** (Deployments → ⋯ → Redeploy).

**Integración opcional:** Vercel Marketplace → Supabase. Crea o enlaza el proyecto y carga las variables sola. Revisa que los nombres coincidan con los de `.env.example`.

## 5. Verificar
```bash
APP=https://<tu-app>.vercel.app
curl $APP/api/health                 # supabase: configured, nasaApiKey: custom
curl $APP/api/notes                  # [] o una lista
curl -H "Authorization: Bearer $CRON_SECRET" $APP/api/cron/keepalive   # ok: true
```
- Vercel → Settings → **Cron Jobs**: tiene que aparecer `/api/cron/keepalive` (diario).
- Abre la URL en **incógnito y en el celular**: así la ven los jueces.
- (Opcional) Pide la clasificación de tu dominio en https://www.fortiguard.com/faq/wfratingsubmit para que los filtros de red de los jueces no la bloqueen.

## 6. Datos estáticos grandes
| Tamaño | Dónde |
|---|---|
| Menos de unos 20 MB, lo lee todo el mundo | `public/data/` (lo sirve la CDN de Vercel) |
| Hasta 50 MB por archivo, 1 GB en total | **Supabase Storage**, bucket público |
| Consultas por filtro, fecha o espacio | Tabla en Supabase (+ `postgis` si hace falta) |
| Más grande | Pre-agrega o recorta: no lo sirvas crudo |

Pre-procesa en `notebooks/` con Python y sube solo el resultado (ver `notebooks/README.md`).

## 7. Mantenerlo vivo hasta enero
- El jurado evalúa hasta el **26 de enero de 2027**. Supabase Free **se pausa después de 7 días sin actividad**: el cron diario de `vercel.json` lo evita. Revisa una vez por semana que el proyecto siga "Active".
- No borres el proyecto de Vercel ni cambies la URL después de entregar.
- Exporta un backup: el plan Free no tiene backups. Usa `pg_dump "<connection string>" -f backup.sql` con la cadena de **Connect → Session pooler**.

## 8. Problemas frecuentes
| Síntoma | Solución |
|---|---|
| `missing env vars` en producción | Faltan las variables en Vercel o se agregaron después del build → Redeploy |
| `[]` aunque hay filas | Falta una política RLS `select` |
| `new row violates row-level security policy` | Falta la política `insert` |
| 429 de api.nasa.gov | Configura `NASA_API_KEY` |
| 504 / timeout en una API route | Superaste el límite de duración de la función (Hobby: 300 s): pre-procesa o cachea |
| Build falla solo en Vercel | Corre `pnpm build` local; revisa la versión de Node en Settings → General |
| Proyecto Supabase "Paused" | Dashboard → Restore. Revisa que el cron esté activo |

## Alternativas (si algo no te sirve)
- **API en Python (FastAPI):** Render Free (se duerme a los 15 minutos de inactividad) o funciones Python en Vercel (con límites de tamaño). Ver la tabla "qué usar para qué" en `docs/resources/ofertas-y-free-tiers.md`.
- **Sitio 100 % estático:** GitHub Pages o Cloudflare Pages.
