@AGENTS.md

# Notas específicas para Claude Code

- Las instrucciones compartidas con todos los agentes (Claude Code, OpenCode y otros) están en `AGENTS.md` (importado arriba). OpenCode no lee este archivo: lo que deba valer para ambos va en `AGENTS.md`. Las **reglas del evento** de esa sección son obligatorias, en especial: **no idear la solución del challenge** y llevar el registro de uso de SI en `docs/SI_USAGE.md`.
- Las skills se cargan solas desde `.claude/skills/`. Al empezar una sesión de trabajo en el hackathon, carga `space-apps-challenge` y, según la tarea, la que corresponda de la tabla de `AGENTS.md`.
- Para Next.js, el conocimiento de referencia está en `node_modules/next/dist/docs/` (no hay skill de "best practices": Next.js la reemplazó por esas docs). Para verificar cambios en el navegador, usa `next-dev-loop` o `webapp-testing`.
- Skills de Vercel sin licencia de redistribución, que no se incluyen en el repo: si las quieres en tu máquina, `npx skills add vercel-labs/agent-skills --skill deploy-to-vercel --skill react-best-practices --skill web-design-guidelines`.
- MCP útiles (opcionales, configúralos en tu máquina): Supabase MCP (https://supabase.com/docs/guides/getting-started/mcp), Vercel MCP (https://vercel.com/docs/mcp) y Context7 para docs de librerías.
- Antes de decir que algo está listo: `pnpm lint && pnpm build`. Para cambios de UI, mira la página en el navegador.
- Commits: pequeños y descriptivos. No subas `.env.local` ni datos crudos.
