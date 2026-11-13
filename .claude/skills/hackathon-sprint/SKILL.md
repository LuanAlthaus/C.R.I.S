---
name: hackathon-sprint
description: Plan de ejecución para las 36–48 h del hackathon — cronograma por bloques, roles del equipo, definición del MVP, recorte de alcance, gestión de riesgos (APIs caídas, datos pesados), flujo de git y checkpoints. Úsala al arrancar el hackathon, cuando el equipo esté atrasado o cuando haya que decidir qué cortar.
---

# Sprint de hackathon

## Cronograma recomendado (sáb 9:00 → dom 23:59, hora local)
| Bloque | Objetivo | Salida |
|---|---|---|
| Sáb 9–11 | Elegir desafío; **el equipo** genera la idea (sin SI) y la encuadra (`data-biz`) | `docs/PROJECT.md`, checklist de requisitos del desafío |
| Sáb 11–13 | **Spike de datos**: bajar UN dato real y mostrarlo feo en pantalla | Fin a fin funcionando: dato → API → UI |
| Sáb 13 | **Deploy temprano** a Vercel (ver `deploy-vercel-supabase`) | URL pública que funciona |
| Sáb 14–22 | Feature principal (el "momento wow") | Demo de la historia principal |
| Sáb 22 | Checkpoint: ¿la demo se entiende sin explicar? Recortar | Backlog recortado |
| Dom 9–15 | Pulido de UI (`frontend-design`), metodología, estados de error | App presentable |
| Dom 15–18 | Demo: video de 30 s o menos, o 7 slides como máximo, en inglés (`pitch-and-submission`) | Demo subida y pública |
| Dom 18–21 | Entrega en la plataforma; README final | Proyecto enviado (no esperes al cierre) |
| Dom 21–23:59 | Margen para imprevistos | — |

Dormir es parte del plan: un equipo descansado entrega mejor el domingo.

## Roles (equipo de 3 a 6)
- **Datos:** notebook, pre-proceso, verificación científica.
- **Front:** UI, mapas o gráficos.
- **Back/infra:** API routes, Supabase, deploy (puede ser la misma persona que el front).
- **Producto/pitch:** encuadre, guion, video, slides, prueba de usabilidad con alguien de afuera. Es tan importante como el código.

## Definir el MVP
El equipo escribe la **única historia** que la demo debe contar (formato: *usuario + acción + resultado que ve*). Ejemplo de otro dominio:
> "Una enfermera escolar abre la app, ve que mañana habrá alerta de mala calidad del aire en su barrio y descarga el aviso para las familias."

Todo lo que no sirve a esa historia va al backlog. Prioriza con MoSCoW (Must, Should, Could, Won't) y deja los Must en 3.

## Riesgos y planes B
| Riesgo | Plan B |
|---|---|
| API lenta o caída durante la demo | Snapshot de las respuestas en `public/data/*.json` y una variable `USE_SNAPSHOT=1` |
| Datos demasiado pesados | Recortar área y período; agregar; pre-procesar offline |
| Earthdata o AppEEARS tarda | Lanzar los pedidos el sábado a primera hora; trabajar con una muestra mientras tanto |
| LLM sin cuota | Respuestas cacheadas para las preguntas de la demo |
| Se rompe main antes de entregar | Tag `v-demo` con la última versión buena; Vercel permite promover un deploy anterior |

## Git en equipo
- `main` siempre desplegable. Ramas cortas `feat/*` y PRs pequeños; Vercel crea un Preview por PR.
- Commits frecuentes. No subir `.env.local` ni datos crudos pesados (están en `.gitignore`; usa `data/raw/`).
- La persona de datos sube **solo** los resultados livianos a `public/data/`.

## Checkpoints rápidos (cada 4 h)
1. ¿La URL de producción funciona en el celular?
2. ¿La historia principal se puede mostrar de punta a punta?
3. ¿Qué cortamos si quedan 6 horas?
