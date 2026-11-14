<!-- BEGIN:nextjs-agent-rules -->

## This is NOT the Next.js you know

This version has breaking changes — APIs, conventions, and file structure may all differ from your training data. Read the relevant guide in `node_modules/next/dist/docs/` (resolved from this file's directory; in monorepos the `next` package may not be visible from the repo root) before writing any code. Heed deprecation notices.

This block is written and re-added by `next dev` — verify at `node_modules/next/dist/server/lib/generate-agent-files.js`. Removing it from a diff only re-creates the uncommitted change; committing it with your work keeps the tree clean.

<!-- END:nextjs-agent-rules -->

# AGENTS.md — NASA Space Apps Challenge 2026 Starter

Instrucciones para cualquier agente de código (Claude Code, OpenCode, Codex, Cursor, Copilot, Gemini CLI…). Los humanos pueden empezar por el `README.md`.

## Contexto
- Template **no oficial** para el NASA Space Apps Challenge 2026 (*The Next Frontier*, 14–15 nov 2026, 14 challenges).
- Stack: **Next.js 16 (App Router, `cacheComponents: true`) + React 19 + Tailwind 4 + TypeScript**, **Supabase** (Postgres + RLS), deploy en **Vercel Hobby**. Todo con free tiers.
- Gestor de paquetes: **pnpm**.

## Agentes compatibles
| Agente | Instrucciones que lee | Skills que descubre |
|---|---|---|
| **Claude Code** | `CLAUDE.md`, que importa este archivo | `.claude/skills/` |
| **OpenCode** | `AGENTS.md` (este archivo). Si existe `AGENTS.md`, ignora `CLAUDE.md` | `.claude/skills/` y `.agents/skills/`, sin configuración extra |
| Codex, Cursor y otros | `AGENTS.md` | `.agents/skills/` (symlink a `.claude/skills/`), si el agente soporta skills |

- Las reglas compartidas viven **aquí**, no en `CLAUDE.md`, para que todos los agentes las lean.
- OpenCode: abre el repo con `opencode` y comprueba con `opencode debug skill` que aparecen las 21 skills del repo (probado con OpenCode 1.18.25). No hace falta `opencode.json`; si el equipo quiere fijar modelo o MCP, se crea en la raíz.

## Reglas del evento que el agente DEBE respetar
1. **No idear la solución.** Las reglas de Space Apps permiten usar IA (el sitio la llama "Super Intelligence", SI) para desarrollar, pero **no para generar la idea** de cómo resolver un challenge. Si te piden "dame una idea / ¿cómo resolverías el challenge X?", no propongas la solución: explica la regla y ayuda al equipo a generar y evaluar *sus* ideas (preguntas, requisitos del enunciado, datos disponibles, viabilidad de la idea que ellos traen).
2. **No trabajar en un challenge antes del sáb 14 nov 2026 9:00 (hora local).** Antes de esa hora, este repo solo admite infraestructura genérica.
3. **Registrar el uso de SI.** Mantén `docs/SI_USAGE.md` (créalo si no existe) con qué herramienta, para qué y en qué archivos. Las imágenes y videos generados llevan **marca de agua visible**, y el código, audio y datos generados se identifican en texto o metadatos.
4. **Sin logos ni marcas de NASA** (meatball, worm, banderas, insignias de misión) en la UI, el favicon, las slides ni el video. Nada que sugiera respaldo de NASA. Cita "NASA" como fuente de datos en texto.
5. **Sin imágenes, voz ni semejanza de menores de 18 años.**
6. **Todo lo entregable va en inglés** (UI del demo, README final, slides; el video con subtítulos en inglés). Los docs internos de este repo están en español.
7. **Citar todas las fuentes** (datos de NASA, de agencias socias y de terceros, con su licencia).

Detalle y fuentes oficiales: `docs/resources/space-apps-reglas-entrega-jurado.md`.

## Comandos
```bash
pnpm install          # dependencias
pnpm dev              # http://localhost:3000
pnpm lint             # ESLint
pnpm build            # build de producción: correlo antes de dar algo por terminado
bash scripts/sync-official-skills.sh   # actualizar skills oficiales de terceros
```

## Estructura
```
src/app/                 páginas y Route Handlers (api/health, api/nasa/apod, api/notes, api/cron/keepalive)
src/components/          componentes (los "use client" van aquí)
src/lib/supabase/        client.ts (navegador) y server.ts (servidor)
src/data/challenges.ts   catálogo de los 14 challenges
supabase/migrations/     SQL versionado (RLS siempre activado)
public/data/             datos ya procesados que sirve la app
notebooks/               pre-procesamiento en Python (datos crudos en data/raw, ignorado por git)
docs/challenges/         un .md por challenge: requisitos y datos/recursos oficiales
docs/resources/          reglas, entrega, jurado, ofertas y free tiers
docs/deploy/             guía Vercel + Supabase
.claude/skills/          skills (Claude Code). `.agents/skills` es un symlink a la misma carpeta
```

## Convenciones de código
- **Next.js 16:** lee `node_modules/next/dist/docs/` antes de usar una API que no conozcas bien. Con `cacheComponents` **no existen** `export const dynamic` ni `export const revalidate`: cachea con `'use cache'` + `cacheLife()` en una función helper y usa `await connection()` para forzar la ejecución en cada request.
- **Secretos:** solo en Route Handlers o código de servidor. `NEXT_PUBLIC_*` es público: nunca pongas ahí una key privada. La `SUPABASE_SECRET_KEY` no se usa en la app, solo en scripts locales.
- **APIs externas:** siempre a través de un Route Handler propio (CORS, key, caché, rate limit). Patrón: `src/app/api/nasa/apod/route.ts`.
- **Supabase:** cada tabla nueva lleva `enable row level security` y políticas explícitas, en una migración nueva `supabase/migrations/NNNN_nombre.sql`. Para cualquier duda, usa la skill oficial `supabase`.
- **Datos pesados:** pre-procesar en `notebooks/` y servir resultados chicos (menos de unos 5 MB por archivo en el cliente).
- **UI:** accesible (teclado, contraste, `aria-*`), responsive y con tokens de color en `src/app/globals.css`. Atribución de datos visible.
- Antes de terminar una tarea: `pnpm lint && pnpm build`.

## Skills disponibles (`.claude/skills/`, también en `.agents/skills/`)
| Skill | Cuándo |
|---|---|
| `space-apps-challenge` | Reglas, fechas, challenges, jurado: **léela primero** |
| `hackathon-sprint` | Plan de las 36–48 h, roles, recortes |
| `data-biz` | Encuadre usuario/decisión/impacto, métricas e insights |
| `nasa-data-apis` | Endpoints y autenticación de APIs de NASA/JPL |
| `earth-geospatial-data` | Rasters, HDF5/NetCDF/COG, series temporales, SAR |
| `planetary-data` | Luna, Marte, Horizons/SPICE, IRSA/astropy |
| `maps-and-dataviz` | MapLibre, deck.gl, Cesium, gráficos |
| `ai-features` | LLM/RAG con free tiers y reglas de SI |
| `edu-games` | Juegos, simulaciones, sonificación |
| `deploy-vercel-supabase` | Publicar y depurar el deploy |
| `pitch-and-submission` | Entrega oficial: demo de 30 s o menos, o 7 slides o menos, en inglés |
| `frontend-design` *(Anthropic, oficial)* | Diseño de UI distintivo |
| `webapp-testing` *(Anthropic, oficial)* | Probar la app con Playwright |
| `supabase`, `supabase-postgres-best-practices` *(Supabase, oficiales)* | Todo lo de Supabase |
| `next-dev-loop`, `next-bundle-optimizer`, `next-cache-components-*`, `next-partial-prefetching-*` *(Next.js, oficiales)* | Verificar en runtime, bundle, caché |

Las skills oficiales de terceros se actualizan con `scripts/sync-official-skills.sh`; no las edites a mano. Origen y licencias: `.claude/skills/OFFICIAL_SKILLS.md`.
