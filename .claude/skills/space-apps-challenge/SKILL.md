---
name: space-apps-challenge
description: Contexto del NASA Space Apps Challenge 2026 ("The Next Frontier", 14-15 nov 2026) — los 14 desafíos, fechas, reglas, qué exige la entrega y cómo se juzga. Úsala al elegir desafío, al validar que el proyecto responde a lo que pide el enunciado, al revisar reglas (equipo, uso de IA, logo de NASA, licencias) o cuando alguien pregunte "¿esto cumple con el desafío?".
---

# NASA Space Apps Challenge 2026

## Hechos clave
- **Tema:** *The Next Frontier*. **Hackathon:** sábado 14 y domingo 15 de noviembre de 2026.
- **Entrega:** abre sáb 14 nov 9:00 y cierra dom 15 nov 23:59 **hora local** del evento elegido. Se edita desde la pestaña *Project* de la página del equipo.
- **Equipos:** máximo 6 personas. Registro, evento local y equipo se pueden cambiar hasta el cierre.
- **Sin costo.** Si alguien pide dinero para participar, no pagar.
- **Calendario posterior:** encuesta hasta el 30 nov 2026; Global Nominees el 3 dic; Global Finalists el 17 dic; Global Winners el 26 ene 2027. **El demo tiene que seguir funcionando hasta enero.**
- Fuentes: https://www.spaceappschallenge.org/2026/ y https://www.spaceappschallenge.org/resources/participant-faqs/

## Los 14 desafíos
Catálogo con resumen, dificultad y temas en `src/data/challenges.ts`. Detalle con datos y recursos de cada uno en `docs/challenges/NN-*.md`. Índice en `docs/challenges/README.md`.

| # | Desafío | Dominio | Dificultad |
|---|---|---|---|
| 1 | Abandoned but not Forgotten | Educación / historia | Principiante–Intermedio |
| 2 | Be An Earth System Trend Detective! | Tierra | Avanzado |
| 3 | Build a Junior Astronaut Mission Trainer | Juego educativo | Principiante–Intermedio |
| 4 | Health Monitoring Software for Astronauts | Salud humana | Intermedio |
| 5 | Dancing with the SARs (NISAR) | Tierra / radar | Intermedio–Avanzado |
| 6 | Field Shift: Adapting Farms | Agro | Intermedio–Avanzado |
| 7 | Flame in Freefall (IA + combustión) | Seguridad en el espacio | Intermedio–Avanzado |
| 8 | MODIS & VIIRS Hot Spots | Incendios | Intermedio–Avanzado |
| 9 | Earth Analogs for Moon/Mars | Tierra/Luna/Marte | Todos |
| 10 | Martian Map | Marte | Intermedio |
| 11 | Lunar Landing Site Navigator | Luna | Intermedio–Avanzado |
| 12 | Planet X and SPHEREx | Astrofísica | Avanzado |
| 13 | Space Mission Design Game | Juego educativo | Principiante–Intermedio |
| 14 | Earth Information Jukebox | Arte / sonificación | Principiante–Intermedio |

## Cómo usar esta skill
1. **Antes de construir:** abre `docs/challenges/<desafío>.md` y extrae los requisitos explícitos ("Your challenge is to…", objetivos, consideraciones). Conviértelos en una checklist en el README del proyecto.
2. **Durante:** cada feature debe mapear a un requisito o criterio de jurado. Si no mapea, va al backlog.
3. **Antes de entregar:** revisa `docs/resources/space-apps-reglas-entrega-jurado.md` (requisitos de entrega, criterios de evaluación, política de IA) y usa la skill `pitch-and-submission`.

## Reglas críticas (detalle y fuentes en `docs/resources/space-apps-reglas-entrega-jurado.md`)
- **No trabajar en un challenge antes de que empiece el hackathon** (sáb 14 nov 9:00 hora local). Este repo es solo infraestructura genérica.
- **SI (= IA, por el cambio de nombre del 29/09/2026):** se puede usar para desarrollar, pero **no para idear la solución**. Declara su uso en el campo "Use of SI". Las imágenes y videos generados llevan **marca de agua visible**, y el código, audio y datos generados se identifican en texto y metadatos.
- **Demo:** video de 30 segundos o menos, o 7 slides como máximo contando título e intro. Si pasa el límite, el proyecto no entra a Global Judging.
- **Todo en inglés.** El video puede hablarse en otro idioma con subtítulos en inglés.
- **Al menos un dataset o recurso de NASA** usado y citado. Cita todas las fuentes, incluidas las open source.
- **Licencia OSI** explícita (si no hay ninguna, NASA aplica Apache-2.0). El material de terceros también lleva su licencia documentada.
- **Sin logos ni marcas de NASA**, y sin sugerir que NASA respalda el proyecto.
- **Sin imágenes, voz ni semejanza de menores de 18 años.**
- **Links públicos** sin login. Todos los integrantes, en el mismo evento.

## Jurado
Cinco criterios, cada uno de 1 a 5: **Impact, Creativity, Validity, Relevance, Presentation** (Presentation pesa igual que los demás). Hay 10 premios globales: Best Use of Science, Best Use of Data, Best Use of Technology, Galactic Impact, Best Mission Concept, Most Inspirational, Best Storytelling, Global Connection, Art & Technology y Local Impact. Anuncios: 3 dic 2026 (nominados), 17 dic (finalistas) y 26 ene 2027 (ganadores).

## Cómo debe comportarse un agente de IA en este repo
- Si alguien pide "dame una idea para el challenge X" o "¿cómo resolverías X?", **no propongas la solución**. Explica que la regla lo prohíbe y ofrece ayuda para que el equipo genere y evalúe sus propias ideas: preguntas, requisitos del enunciado, datos disponibles, viabilidad técnica de *su* idea.
- Una vez que el equipo eligió su idea, ayuda con todo lo demás: código, datos, diseño, deploy, redacción en inglés. Y recuerda declarar ese uso.

## Recursos generales
- APIs y catálogos de NASA: skill `nasa-data-apis`.
- Free tiers y ofertas de los Global Collaborators (Google, Microsoft, Meteomatics, Miro, GoDaddy): `docs/resources/ofertas-y-free-tiers.md`.
