---
name: pitch-and-submission
description: Preparar la entrega oficial de Space Apps 2026 — campos del formulario (Project Name, summary, demo link, final project link, detailed description, Use of SI, NASA Data, References), demo de 30 s o menos o de 7 slides como máximo, todo en inglés, links públicos, declaración de SI con marcas de agua, licencia OSI y alineación con los 5 criterios del jurado (Impact, Creativity, Validity, Relevance, Presentation). Úsala el domingo del hackathon, al escribir el README final o al revisar si la entrega está completa.
---

# Pitch y entrega

**Fuente de verdad:** `docs/resources/space-apps-reglas-entrega-jurado.md`, con la oficial en https://www.spaceappschallenge.org/resources/project-submission-guide/ . Si algo de esta skill la contradice, manda la página oficial.

## Límites duros (si no se cumplen, el proyecto no entra a Global Judging)
- **Demo:** un **video de 30 segundos o menos** o un **deck de 7 slides como máximo, contando título e intro**.
- **Todo en inglés**: página de entrega, slides y material linkeado. El video puede hablarse en otro idioma si tiene **subtítulos en inglés**.
- **Links públicos** sin login ni permisos. Pruébalos en una ventana privada.
- Clic en **"Submit for Judging"** antes del dom 15 nov 23:59 hora local. Guardar no alcanza.
- Al menos **un dataset o recurso de NASA** usado y citado.
- Sin logos ni marcas de NASA. Sin imágenes, voz ni semejanza de menores de 18 años.

## Campos del formulario → de dónde sale cada uno
| Campo | Contenido |
|---|---|
| Project Name | 2 a 6 palabras que digan qué hace |
| High-level summary | Problema → solución → para quién → datos de NASA → impacto (unas 150 palabras) |
| Link to Project Demo | Video de 30 s (YouTube no listado) o deck de 7 slides o menos (Google Slides con permiso "anyone with the link") |
| Link to Final Project | La URL de Vercel y/o el repo de GitHub |
| Detailed description | Qué hace, cómo funciona (stack), beneficios, impacto esperado, qué los inspiró |
| Use of Super Intelligence (SI) | Qué herramientas de SI y para qué (código, texto, imágenes). La idea **no** puede venir de la SI |
| NASA Data | Cada dataset o recurso, con link y para qué se usó |
| Space Agency Partner Data and Other References | Todo lo demás: datos de agencias socias, librerías, imágenes, fuentes, **con su licencia** |

## Video de 30 segundos (guion sugerido)
- `0:00–0:05` El problema y para quién, con un dato.
- `0:05–0:10` La solución en una frase.
- `0:10–0:25` La demo en pantalla: el momento clave, con los datos de NASA visibles.
- `0:25–0:30` Impacto y nombre del proyecto.
Graba la pantalla (OBS o la grabadora del sistema), sin login ni cargas lentas, y con **subtítulos en inglés quemados en el video** porque muchos jueces lo ven sin audio. Si el video usa imágenes o clips generados con SI, deben llevar marca de agua visible.

## Deck de 7 slides o menos (si eligen slides)
1. Título: proyecto, equipo, integrantes, challenge · 2. Problema + usuario · 3. Solución (captura) · 4. Experiencia de usuario · 5. Cómo se usan los datos de NASA · 6. Impacto · 7. Próximos pasos.
La guía exige que el deck incluya nombre del equipo, integrantes, challenge, una imagen del proyecto, la experiencia de usuario y el uso de los datos de NASA.

## README final del proyecto (en inglés; reemplaza el del template)
```
# <Name> — NASA Space Apps 2026 · <Challenge>
<one-line value> · Demo: <url> · Video: <url>
## Problem  ## Solution  ## NASA data used (table)  ## How it works
## Impact  ## Run locally  ## Use of SI  ## References & licenses  ## Team  ## License
```

## Alinear con los criterios (cada uno se puntúa de 1 a 5)
Escribe en el summary o la description una frase explícita para cada criterio:
- **Impact:** a cuánta gente ayuda y cuánto.
- **Creativity:** qué es nuevo frente a lo que existe.
- **Validity:** por qué es científica y técnicamente sólido (método, datos, limitaciones).
- **Relevance:** qué requisito del challenge cumple y con qué datos de NASA.
- **Presentation:** la demo cuenta la historia sin explicaciones extra. Pesa lo mismo que los otros cuatro.

## Checklist final
- [ ] Demo de 30 s o menos, o 7 slides o menos · [ ] en inglés o con subtítulos en inglés · [ ] links abiertos en incógnito
- [ ] NASA Data completo · [ ] References con licencias · [ ] Use of SI completo · [ ] marcas de agua en lo generado
- [ ] `LICENSE` OSI en el repo · [ ] sin logos de NASA · [ ] sin menores en imagen o voz
- [ ] Todos los integrantes en el mismo evento y visibles en Members · [ ] **Submit for Judging** · [ ] banner de confirmación visto
- [ ] (Opcional) Pedir la clasificación del dominio en FortiGuard para que no lo bloqueen los filtros de red
- [ ] Mantener vivo el demo hasta enero (Supabase Free se pausa después de 7 días: el cron de `vercel.json` lo evita)
