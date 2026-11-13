# Guion de video corto y caso de uso

Dos partes:
- **A. Plantillas para el proyecto del equipo:** se completan desde el sáb 14 nov 9:00 con la idea del equipo. Por regla oficial, la IA no puede idear la solución.
- **B. Guion y caso de uso para presentar este starter template:** ya redactados.

---

## A. Plantillas para tu proyecto del challenge

### A1. Guion de demo: 30 segundos como máximo, en inglés o con subtítulos en inglés
Si pasa de 30 s, el proyecto no entra a Global Judging. Unas 65–75 palabras de narración caben en 30 s.

| Tiempo | Pantalla | Narración (EN) | Subtítulo |
|---|---|---|---|
| 0:00–0:05 | Imagen o dato del problema | "Every year, **[dato del problema]** affects **[quién]**." | igual |
| 0:05–0:10 | Logo o nombre del proyecto (sin logos de NASA) | "**[Nombre]** helps **[usuario]** **[verbo + resultado]**." | igual |
| 0:10–0:22 | Captura de la app: la acción principal en vivo | "Pick **[entrada]**, and it shows **[salida clave]** using NASA's **[dataset]**." | + nombre del dataset en pantalla |
| 0:22–0:27 | El resultado, con zoom | "So **[usuario]** can **[decisión]** **[X días / % / horas]** sooner." | igual |
| 0:27–0:30 | Nombre, URL y equipo | "**[Nombre]**: built for NASA Space Apps 2026." | URL |

Checklist del video: subtítulos en inglés quemados en el video · sin login ni pantallas de carga · datos de NASA nombrados en pantalla · nada generado con SI sin marca de agua · sin menores en imagen ni voz · sin logo de NASA · subido como "unlisted" y probado en incógnito.

### A2. Caso de uso (para la descripción detallada y el README)
```
Nombre del caso:        [Verbo + objeto, p. ej. "Planificar ___"]
Actor principal:        [rol concreto]
Contexto / disparador:  [cuándo y por qué abre la herramienta]
Precondiciones:         [qué tiene o sabe antes]
Flujo principal:
  1. El usuario ___
  2. El sistema consulta ___ (dataset de NASA: ___)
  3. El sistema muestra ___
  4. El usuario decide ___
Flujos alternativos:    [sin datos para la zona o fecha / API caída → snapshot]
Resultado (postcondición): [qué sabe o hace ahora que antes no podía]
Métrica de éxito:       [tiempo ahorrado, alerta anticipada, personas alcanzadas…]
Datos de NASA usados:   [producto, resolución, período]
Requisito del challenge que cubre: [cita del enunciado en docs/challenges/NN-*.md]
```

---

## B. Presentación del starter template

### B1. Guion de 30 segundos (EN, para compartir el template)
| Tiempo | Pantalla | Narración |
|---|---|---|
| 0:00–0:05 | Reloj del hackathon: "48 h" | "Space Apps gives you 48 hours. Don't spend the first six on setup." |
| 0:05–0:12 | `gh repo create --template` → `pnpm dev` | "This free starter gives you Next.js and Supabase, ready to deploy on Vercel in one click." |
| 0:12–0:20 | Página de inicio con los 14 challenges → un doc de recursos | "All 14 challenges, with their official NASA datasets and APIs, documented and linked." |
| 0:20–0:26 | Carpeta `.claude/skills` y una sesión de Claude Code | "Plus agent skills for NASA data, maps, deploy and pitch, with the event rules built in." |
| 0:26–0:30 | URL del repo | "Clone it, bring your idea, and ship. Link below." |

### B2. Versión de 60–90 s (para redes, en español)
1. **Gancho (0–8 s):** "El Space Apps dura 48 horas y la mayoría de los equipos pierde las primeras seis armando el proyecto. Este template te las ahorra."
2. **Qué es (8–25 s):** "Es un repo público y gratuito con Next.js 16 y Supabase, listo para desplegar en Vercel sin pagar nada. Trae ejemplos de cómo llamar a las APIs de NASA sin exponer tus claves y un cron que mantiene vivo tu demo hasta que el jurado termine en enero."
3. **Los 14 challenges (25–40 s):** "Cada challenge tiene su ficha: qué pide, qué datos de NASA y de las agencias socias ofrece, y cómo acceder a ellos por API."
4. **Skills para agentes (40–60 s):** "Si programas con Claude Code, Codex o Cursor, el repo trae skills: datos de NASA, geoespacial, Luna y Marte, mapas, despliegue y armado del pitch, más las oficiales de Supabase y Next.js. Y respeta las reglas: la IA te ayuda a construir, pero la idea la pone tu equipo."
5. **Cierre (60–75 s):** "Úsalo como template, despliega en 20 minutos y dedica el hackathon a lo que importa. El link está en la descripción."

### B3. Caso de uso del template
**Nombre:** Arrancar el proyecto de un equipo y tenerlo desplegado en la primera hora del hackathon.

| | |
|---|---|
| **Actor** | Equipo de 3–6 personas (datos, front, producto) en un evento local de Space Apps |
| **Disparador** | Sábado 9:00: abre la entrega y el equipo ya eligió su challenge y su idea |
| **Precondiciones** | Cuentas de GitHub, Vercel y Supabase; key de api.nasa.gov |

**Flujo principal**
1. La líder técnica crea el repo del equipo con **"Use this template"** y suma a los integrantes.
2. El equipo lee `docs/challenges/NN-*.md` de su challenge y convierte los requisitos en una checklist.
3. Back/infra crea el proyecto en Supabase, aplica `supabase/migrations/` y conecta el repo en Vercel con las variables de `.env.example` (unos 20 minutos, siguiendo `docs/deploy/vercel-supabase.md`).
4. Verifica `/api/health` en la URL pública: **demo online antes de las 10:30**.
5. La persona de datos usa `notebooks/` y la skill `nasa-data-apis` para bajar el primer dataset y dejar un JSON liviano en `public/data/`.
6. El front reemplaza `src/app/page.tsx` y usa `maps-and-dataviz` y `frontend-design`. Cada PR genera una URL de preview.
7. Producto completa `docs/PROJECT.md` (skill `data-biz`) y va llenando `docs/SI_USAGE.md`.
8. El domingo, la skill `pitch-and-submission` guía el video de 30 s y el formulario. El equipo envía antes de las 21:00.

**Flujos alternativos**
- La API de NASA está lenta → se sirve el snapshot de `public/data/`.
- Supabase se pausa durante la evaluación → el cron diario lo evita; si igual pasa, se hace "Restore" desde el dashboard.

**Resultado:** el equipo dedica más de 40 de las 48 h al problema y no a la infraestructura, y el demo sigue online hasta el anuncio de ganadores (26 ene 2027).

**Métricas:** tiempo hasta la primera URL pública (menos de 1 h) · US$ 0 de infraestructura · entrega completa sin rechazos por formato.
