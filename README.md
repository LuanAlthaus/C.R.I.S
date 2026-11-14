# NASA Space Apps Challenge 2026 — Starter Template

Template **no oficial** y gratuito para equipos del [NASA Space Apps Challenge 2026](https://www.spaceappschallenge.org/2026/) (*The Next Frontier*, **14–15 de noviembre de 2026**).

Incluye:
- **App base** Next.js 16 + Supabase lista para desplegar en Vercel (**US$ 0**), con ejemplos de proxy a APIs de NASA, lectura y escritura en la base con RLS, y un cron que evita que el demo se apague antes de que termine la evaluación.
- **Skills para agentes de código** (Claude Code, OpenCode, Codex, Cursor…): las propias del repo (datos de NASA, geoespacial, planetario, data-biz, mapas, IA, juegos, deploy, pitch, sprint) y las **oficiales** de Supabase, Next.js y Anthropic (frontend-design).
- **Los 14 challenges documentados**, con los datos y recursos oficiales de NASA y de las agencias socias.
- **Reglas, requisitos de entrega, criterios del jurado y free tiers**, verificados el 13 nov 2026.
- `CLAUDE.md` y `AGENTS.md` con las reglas del evento incorporadas (por ejemplo: la IA no idea la solución).

> ⚠️ Este repo es **infraestructura genérica**. Las reglas prohíben trabajar en un challenge antes del **sáb 14 nov 9:00 (hora local)** y **usar IA para idear la solución**. Ver [reglas](docs/resources/space-apps-reglas-entrega-jurado.md).

## Empezar
```bash
# 1. Crear tu repo desde este template (o botón "Use this template" en GitHub)
gh repo create mi-equipo --public --template marcosferr/nasa-space-apps-starter-template --clone
cd mi-equipo && pnpm install

# 2. Variables
cp .env.example .env.local     # Supabase URL + publishable key, NASA_API_KEY

# 3. Correr
pnpm dev                       # http://localhost:3000  ·  /api/health
```
Deploy paso a paso: **[docs/deploy/vercel-supabase.md](docs/deploy/vercel-supabase.md)**.

## Mapa del repo
| Ruta | Qué hay |
|---|---|
| [`docs/challenges/`](docs/challenges/README.md) | Los 14 challenges: objetivos, datos y recursos oficiales con links |
| [`docs/resources/space-apps-reglas-entrega-jurado.md`](docs/resources/space-apps-reglas-entrega-jurado.md) | Qué exige la entrega, criterios del jurado, premios, reglas de IA ("SI"), fechas |
| [`docs/resources/ofertas-y-free-tiers.md`](docs/resources/ofertas-y-free-tiers.md) | Global Offers 2026 y límites de Vercel, Supabase, Cloudflare, Gemini, Groq, HF, Render… |
| [`docs/deploy/vercel-supabase.md`](docs/deploy/vercel-supabase.md) | Guía de despliegue gratis |
| [`docs/SI_USAGE.md`](docs/SI_USAGE.md) | Registro de uso de IA para el formulario de entrega |
| [`.claude/skills/`](.claude/skills) | Skills (también en `.agents/skills`, que es un symlink) |
| `src/` | App Next.js (`app/api/*`: health, nasa/apod, notes, cron/keepalive) |
| `supabase/migrations/` | Esquema SQL con RLS |
| `notebooks/` | Pre-procesamiento en Python (earthaccess, xarray, astropy…) |

## Skills
**Propias del repo** (MIT): `space-apps-challenge` · `hackathon-sprint` · `data-biz` · `nasa-data-apis` · `earth-geospatial-data` · `planetary-data` · `maps-and-dataviz` · `ai-features` · `edu-games` · `deploy-vercel-supabase` · `pitch-and-submission`

**Oficiales de terceros, incluidas en el repo** ([orígenes y licencias](.claude/skills/OFFICIAL_SKILLS.md)):
- Supabase ([supabase/agent-skills](https://github.com/supabase/agent-skills), MIT): `supabase`, `supabase-postgres-best-practices`
- Next.js ([vercel/next.js/skills](https://github.com/vercel/next.js/tree/canary/skills), MIT): `next-dev-loop`, `next-bundle-optimizer`, `next-cache-components-adoption`, `next-cache-components-optimizer`, `next-partial-prefetching-adoption`, `next-partial-prefetching-optimizer`. La antigua `next-best-practices` ya no existe: Next.js 16.3+ trae ese conocimiento en `node_modules/next/dist/docs/` y en el bloque de `AGENTS.md`.
- Anthropic ([anthropics/skills](https://github.com/anthropics/skills), Apache-2.0): `frontend-design`, `webapp-testing`

Actualizarlas: `bash scripts/sync-official-skills.sh`.

**Agentes:** Claude Code lee `CLAUDE.md` y `.claude/skills/`. OpenCode lee `AGENTS.md` y descubre las mismas skills sin configurar nada (`opencode debug skill` las lista). Detalle en [`AGENTS.md`](AGENTS.md).
Skills de Vercel Labs ([vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills): `deploy-to-vercel`, `react-best-practices`, `web-design-guidelines`): el repo no declara licencia, así que no se redistribuyen aquí. Instálalas localmente con `npx skills add vercel-labs/agent-skills`.

## Lo esencial de la entrega (resumen)
- Envío desde el sáb 14 nov 9:00 hasta el **dom 15 nov 23:59 hora local**, con el botón **"Submit for Judging"**.
- Demo: **video de 30 s o menos** o **7 slides como máximo** (contando título e intro). **Todo en inglés** (el video puede ir con subtítulos en inglés).
- Al menos **un dato o recurso de NASA**, citado. Todas las fuentes citadas, con licencia.
- Jurado: **Impact, Creativity, Validity, Relevance, Presentation** (cada uno de 1 a 5).
- Uso de IA ("SI") declarado. Imágenes y videos generados con marca de agua. Sin logos de NASA. Sin menores en imagen ni voz. Licencia OSI.

## Cuando empiece el hackathon
1. Reemplaza `src/app/page.tsx` por tu proyecto y este README por el de tu proyecto (en inglés; plantilla en la skill `pitch-and-submission`).
2. Crea `docs/PROJECT.md` con el encuadre (skill `data-biz`).
3. Despliega temprano y despliega seguido.

## Project: Jezero Marswalk Planner — credits
A 3D EVA route planner for a 10 × 10 km area of Jezero crater around the Octavia E. Butler landing site (Bevy → WASM inside this Next.js app). Terrain streams 1 km HiRISE detail tiles (`public/data/jezero/tiles/`) as the camera gets close. Rebuild the data with `python notebooks/jezero_pipeline.py <stage>` and the sim with `bash scripts/build-sim.sh`.

- **AI-generated code (SI):** the data pipeline (`notebooks/jezero_pipeline.py`), the Bevy sim (`sim/`), the web bridge and route handlers (`src/lib/marsSim.ts`, `src/components/MarsSim.tsx`, `src/app/api/nasa/donki-mars/`, `src/app/api/nasa/raw-images/`), `scripts/build-sim.sh` and the UI were written with AI assistance (Claude via omp) from the team's own plan. Details: `docs/SI_USAGE.md`.
- **Human-made data:** hazard training labels in `notebooks/labels/jezero_patches.csv`.
- **3D model:** Perseverance rover, NASA/JPL-Caltech (NASA 3D Resources, `25042_Perseverance.glb`), served as `public/data/jezero/rover.glb` with the NASA/JPL logos, US flag and Mars 2020 insignia painted out of its textures (Space Apps rule: no NASA marks).
- **Data sources:** HiRISE DTM and orthomosaic (NASA/JPL/University of Arizona/USGS) via JPL Mars Trek, plus layers derived from them (slope, DTM roughness, optical texture, sky-view factor); USGS SIM 3464 geologic map (Sun & Stack, 2020); THEMIS day/night IR mosaics (NASA/JPL/ASU) via Mars Trek; Perseverance rover waypoints (NASA/JPL-Caltech) via mars.nasa.gov (https://mars.nasa.gov/mmgis-maps/M20/Layers/json/M20_waypoints.json); NASA InSight weather via api.nasa.gov; DONKI WSA-Enlil (NASA CCMC) via api.nasa.gov; MODIS Terra/Aqua LST via NASA GIBS/Worldview; Perseverance raw images (NASA/JPL-Caltech); limits from NASA-STD-3001. Data: NASA. Not endorsed by NASA.

## Licencia
[MIT](LICENSE). Las skills de terceros conservan sus licencias (`.claude/skills/_licenses/` y el `LICENSE.txt` de cada skill de Anthropic).
Proyecto comunitario, **no afiliado a NASA ni respaldado por NASA**.
