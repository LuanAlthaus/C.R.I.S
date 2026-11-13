# Create Health Monitoring Software for Astronauts on Space Missions
> Crea un software de monitoreo de salud para astronautas en misiones espaciales

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/health-monitoring-software/
- **Dificultad:** Intermediate
- **Temas:** Human Exploration, Software, Space Exploration

## Resumen
Las misiones de larga duración exponen a los astronautas a radiación espacial, aislamiento y confinamiento, gravedad alterada y un entorno cerrado hostil, lo que puede provocar cambios inmunológicos, pérdida ósea, eventos cardiovasculares y problemas de salud conductual. En misiones largas, los astronautas cargan con buena parte de la responsabilidad de detectar estos cambios en sí mismos. El desafío es construir un software de monitoreo de salud que recopile indicadores de salud y permita a los astronautas evaluar su estado y actuar en consecuencia.

## Objetivos / qué piden
Contexto (Background):
- Las misiones son cada vez más ambiciosas: el programa Artemis logró recientemente un sobrevuelo lunar tripulado, y hay planes de enviar humanos a Marte. Misiones más largas implican más exposición a riesgos físicos y mentales.
- El Human Research Program (HRP) de la NASA mapeó los **cinco peligros del vuelo espacial**: radiación espacial, aislamiento y confinamiento, distancia de la Tierra, campos gravitatorios y entornos cerrados hostiles. Pueden causar cambios inmunes, pérdida de densidad ósea, eventos cardiovasculares, nutrición inadecuada, condiciones de salud conductual, etc.
- Los astronautas deben poder evaluarse a sí mismos durante la misión. Indicadores útiles: **presión arterial, frecuencia cardíaca, glucosa en sangre, temperatura corporal, horas de sueño diarias y estado de ánimo**.

Objetivos:
- **Crear "a visual tool that helps astronauts assess their own health during space missions and provides guidance on treatments and health improvements."**
- Entrada de datos por cualquier medio: texto por teclado, clics, imágenes de cámara, etc. Se acepta cualquier dato que represente un indicador de salud (p. ej. glucosa en mg/dL o imágenes de ecografía).
- Usar recursos del HRP y de otras agencias para entender cómo los peligros afectan la salud y decidir qué indicadores sirven para una evaluación rápida. Preguntas guía: ¿qué mediciones se pueden hacer en el espacio? ¿Por qué se necesita un examen? ¿Cómo afectan los peligros a estos indicadores? ¿Cuándo un resultado amerita atención de la tripulación o de los médicos de vuelo en tierra? ¿Qué acciones previenen una condición crítica?
- El diseño de interfaz es clave para presentar métricas y diagnóstico con claridad; se sugieren animaciones e interactividad, por ejemplo **una figura humana con partes del cuerpo clicables** que muestren su función y estado de salud.

## Consideraciones potenciales
- La herramienta podría dar información de salud en distintos entornos: nave, estación espacial, rover o base en otro planeta o luna.
- Podría pedir a los astronautas tomar mediciones o completar acciones periódicamente como parte de la rutina de misión.
- Funcionalidades posibles:
  - Sistema de gamificación para que los astronautas se involucren con tareas que apoyan su salud.
  - Gráficos dinámicos que muestren cómo cambian los indicadores con el tiempo.
  - Código de colores para interpretar el estado de un indicador de un vistazo.
- No asumir que los astronautas saben todo sobre cómo evaluar su salud: dar instrucciones y explicaciones simples sobre mediciones, indicadores y riesgos.

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| 5 Hazards of Human Spaceflight | Información básica del HRP sobre los principales peligros del vuelo espacial, con enlaces a artículos e informes detallados por peligro | Documento / página web | https://www.nasa.gov/hrp/hazards/ |
| Risks of Human Spaceflight | Información básica del HRP sobre los principales riesgos para humanos en misiones, con enlaces a artículos e informes por riesgo | Documento / página web | https://www.nasa.gov/hrp/risks/ |
| Ongoing HRP Research in Space | Artículos sobre investigaciones en curso del HRP | Documento / página web | https://www.nasa.gov/hrp/ongoing-hrp-research/ |

## Recursos de agencias socias
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| CSA: Monitoring Radiation for Astronaut Health | Cómo la CSA ayuda a monitorear la radiación espacial para proteger la salud de astronautas en misiones largas; tecnologías e investigación canadienses de medición de exposición | Documento / artículo web | https://www.asc-csa.gc.ca/eng/news/articles/2026/2026-02-26-monitoring-radiation-astronaut-health.asp |
| CSA: Why Do We Conduct Science Experiments in Space? | Explica por qué el entorno espacial permite experimentos imposibles en la Tierra y cómo la investigación en la ISS avanza el conocimiento sobre salud humana | Documento / página web | https://www.asc-csa.gc.ca/eng/iss/science/why-do-we-conduct-science-experiments-in-space.asp |

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **No hay dataset/API en la lista oficial:** los recursos son documentos del HRP. El "dato" lo genera el usuario (formularios, cámara) o un simulador. Recomendado: generar datos sintéticos realistas de 30-180 días de misión (con deriva de presión, sueño, ánimo) en un script y dejarlo claramente etiquetado como simulado.
- **Datos reales opcionales (sugerencia adicional, no listada en esta página):** NASA OSDR (https://osdr.nasa.gov/bio/repo/) tiene estudios de salud en vuelo espacial y RadLab dosis de la ISS; sirven para justificar umbrales o mostrar contexto, no como datos clínicos de un astronauta.
- **Skills del repo aplicables:** space-apps-challenge, frontend-design, maps-and-dataviz (gráficos), ai-features, edu-games (gamificación), deploy-vercel-supabase, pitch-and-submission, hackathon-sprint; nasa-data-apis solo si suman OSDR/RadLab.
