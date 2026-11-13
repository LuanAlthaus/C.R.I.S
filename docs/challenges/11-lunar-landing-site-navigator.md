# Lunar Landing Site Navigator
> Navegador de sitios de alunizaje

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/lunar-landing-site-navigator/
- **Dificultad:** Advanced, Intermediate (Avanzado, Intermedio)
- **Temas:** Human Exploration, Moon, Software, Space Exploration

## Resumen
Las misiones al polo sur de la Luna enfrentan restricciones extremas de iluminación y comunicación, y planificarlas exige una coordinación cuidadosa entre los equipos de ciencia e ingeniería. Existen herramientas para determinar la posición del Sol y de la Tierra en sitios candidatos de alunizaje, pero sus interfaces complicadas dificultan evaluar la información crítica con eficiencia. El desafío es crear una herramienta o aplicación intuitiva que permita a planificadores de misión, educadores y público en general comparar sitios y fechas de alunizaje rápidamente, visualizando las posiciones del Sol y de la Tierra respecto del horizonte para evaluar el potencial de generación de energía y las ventanas de comunicación directa con la Tierra (direct-to-Earth).

## Objetivos / qué piden
Contexto (Background): el programa Commercial Lunar Payload Services (CLPS) de NASA y otras misiones lunares dependen de elegir sitio y fecha balanceando ciencia e ingeniería. La posición del Sol determina la energía disponible para paneles solares y las condiciones térmicas; la posición de la Tierra determina si es posible comunicarse sin satélites relé. En el polo sur (donde hay hielo de agua) pequeños cambios de ubicación o fecha alteran drásticamente energía y comunicaciones. Las misiones suelen sufrir retrasos que obligan a reevaluar sitio/fecha rápidamente, y herramientas como Quickmap, Moon Trek y JPL Horizons están hechas para análisis detallado y son difíciles para no especialistas.

Piden "create a tool or application that enables rapid evaluation of lunar landing site options". La solución puede incluir:

- **Selección y comparación interactiva de sitios:** lista curada de sitios notables (misiones Apollo, objetivos CLPS, otras misiones comerciales, sitios de programas lunares de otros países) más ingreso de coordenadas lunares personalizadas; comparación lado a lado o secuencial de varios sitios.
- **Flexibilidad de fecha y hora:** elegir fechas/horas de alunizaje y explorar cómo cambian las condiciones a lo largo de distintos días lunares o estaciones.
- **Visualización de posiciones del Sol y la Tierra:** "Display the positions of the Sun and Earth relative to the horizon at a selected landing site on a chosen date"; opcionalmente mostrar el tiempo restante hasta la puesta del Sol (sunset) o de la Tierra (Earthset).
- **Evaluación de energía y comunicaciones:** métricas clave como ángulo de elevación solar, duración de la luz solar y ventanas de visibilidad de la Tierra.
- **Múltiples enfoques de visualización:** mapas 2D, superficies 3D u otras representaciones creativas de la relación espacial sitio–Sol–Tierra.
- **Dinámica temporal:** animaciones o líneas de tiempo que muestren cómo cambian las posiciones a lo largo de un día lunar o varios días.
- **Accesibilidad para distintos niveles:** servir tanto a planificadores que necesitan detalle técnico como a educadores y público curioso.
- Formato libre: web app, app móvil, visualización interactiva, juego educativo u otro, "as long as it makes rapid site evaluation clear and efficient".

## Consideraciones potenciales
*Opcionales, según la página oficial:*

**Enfoque técnico**
- Los datos de efemérides (posiciones y movimientos de cuerpos celestes) pueden calcularse con el JPL Horizons System o usarse datasets precalculados.
- Datos topográficos lunares como los del Lunar Orbiter Laser Altimeter (LOLA) ayudan a crear superficies realistas y cálculos de horizonte precisos.
- La Luna no tiene atmósfera: sin refracción atmosférica, el horizonte lo determinan el terreno local y el borde de la esfera lunar vista desde el sitio.
- El ángulo de elevación solar es la métrica clave para generación de energía; conviene mostrarlo de forma destacada.

**Experiencia de usuario**
- Priorizar la comparación rápida de escenarios (múltiples combinaciones sitio–fecha).
- Comunicar claramente el tiempo restante hasta sunset o Earthset (información operativa crítica).
- Hacer evidentes las diferencias extremas entre condiciones lunares y terrestres.
- Ayudar a entender por qué pequeños cambios de ubicación o fecha tienen grandes impactos.

**Datos y precisión**
- Considerar la topografía real de la Luna, que afecta mucho la posición del horizonte y las ventanas de visibilidad.
- Evaluar si se modelan la libración de la Tierra (wobble) o las variaciones orbitales de la Luna.
- El terreno del polo sur es especialmente complejo (pendientes empinadas y cráteres profundos con sombras extremas).

**Participación e impacto**
- Apoyar la coordinación entre ciencia e ingeniería durante el desarrollo de la misión.
- Responder rápido a cambios de cronograma o problemas de hardware que obligan a ajustar fecha o sitio.
- Servir como herramienta educativa sobre los desafíos de la exploración lunar.

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Quickmap | Herramienta de mapas interactiva (Arizona State University) con imágenes de alta resolución, topografía y otros datasets del Lunar Reconnaissance Orbiter; útil para la topografía del polo sur | Visor web | https://quickmap.lroc.asu.edu/ |
| Moon Trek | Herramienta de visualización 3D de NASA para explorar la superficie lunar, la topografía y el terreno de sitios candidatos | Visor web (con descarga de capas) | https://trek.nasa.gov/moon/ |
| Horizons System | Sistema de efemérides de JPL que calcula posiciones y velocidades de cuerpos celestes, incluyendo Sol y Tierra vistos desde cualquier ubicación en la Luna | API REST / visor web | https://ssd.jpl.nasa.gov/horizons/ |

## Recursos de agencias socias
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| CSA – Moon Exploration | Portal de la Agencia Espacial Canadiense sobre misiones lunares pasadas, actuales y futuras, contribuciones canadienses y participación en Artemis | Documento (sitio informativo) | https://www.asc-csa.gc.ca/eng/astronomy/moon-exploration/ |

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Efemérides vía Horizons API:** `https://ssd.jpl.nasa.gov/api/horizons.api` (sugerencia adicional: endpoint REST oficial de Horizons, sin autenticación). Pedir `EPHEM_TYPE=OBSERVER`, `COMMAND='10'` (Sol) y `'399'` (Tierra), `CENTER='coord@301'` con `COORD_TYPE=GEODETIC` y `SITE_COORD='lon,lat,alt'`, y `QUANTITIES='4'` (azimut/elevación). Devuelve texto/JSON; parsear el bloque entre `$$SOE` y `$$EOE`. No hay CORS amigable para todos los casos: conviene llamarlo desde un route handler del servidor y cachear.
- **Alternativa offline (sugerencia adicional):** SPICE kernels (`de440.bsp`, `moon_pa_de440_200625.bpc`, frames lunares) con `spiceypy` o `skyfield` en Python para precalcular series de elevación/azimut por sitio y exportar JSON. Evita rate limits durante la demo.
- **Topografía / horizonte:** LOLA DEMs polares (sugerencia adicional: PDS Geosciences Node, GeoTIFF/IMG, ~20–240 m/píxel en el polo sur; los archivos pueden pesar cientos de MB). Para el MVP, recortar una región de ~50 km alrededor de cada sitio y precomputar el perfil de horizonte (elevación máxima del terreno por azimut, p. ej. cada 1°) en un notebook. Con eso, "Sol visible" = elevación solar > horizonte local en ese azimut.
- **Gotcha:** sin atmósfera no hay refracción; la elevación geométrica alcanza. La curvatura lunar (radio ~1737.4 km) debe incluirse al calcular el ángulo del terreno lejano. La Tierra desde el polo sur oscila cerca del horizonte por libración, así que una diferencia de pocos grados de terreno cambia todo.
- **Skills del repo aplicables:** `space-apps-challenge`, `planetary-data`, `nasa-data-apis`, `maps-and-dataviz`, `frontend-design`, `deploy-vercel-supabase`, `hackathon-sprint`, `pitch-and-submission` (y `edu-games` si se elige formato educativo).
