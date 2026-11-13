# Interplanetary Survival Guide: Martian Map
> Guía de supervivencia interplanetaria: mapa marciano

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/interplanetary-survival-guide-martian-map/
- **Dificultad:** Intermediate
- **Temas:** Human Exploration, Mars, Planets & Moons, Software, Space Exploration

## Resumen
NASA ha explorado Marte de forma robótica durante décadas, estudiando su ambiente extremo, cartografiando su superficie y recolectando muchos tipos de datos. Lejos de casa, los primeros astronautas que pisen Marte querrán el mejor mapa posible, con información detallada de sus rutas y destinos, actualizaciones sobre las condiciones actuales y los datos necesarios para completar su misión de forma rápida y segura. Tu desafío es crear una vista integrada y en capas de un lugar o una ruta en la superficie marciana que reúna datos de varias misiones científicas de NASA y que pueda ayudar a un explorador humano a planificar y llevar a cabo una caminata marciana ("Marswalk") exitosa, haciendo ciencia nueva y emocionante en el camino.

## Objetivos / qué piden
Contexto (Background):
- Los exploradores humanos pueden hacer ciencia y actividades que los sistemas robóticos no pueden: como se vio en el sobrevuelo lunar de Artemis II, usan el pensamiento crítico para conectar ideas, resolver problemas rápido y dar contexto a las observaciones en tiempo real.
- Hoy la exploración humana se centra en la superficie lunar (más misiones Artemis y la construcción por fases de una Moon Base); parte del rol de las misiones científicas de NASA es preparar el camino para que los humanos visiten nuevos destinos.
- Los datos de las misiones a Marte pintan un cuadro detallado del planeta, pero todavía no están combinados como se necesita para planificar misiones humanas concretas. Herramientas como Mars Trek de JPL y el USGS-NASA Planetary Geologic Mapping Program permiten superponer capas sobre un mapa global, pero "are built for regional and global views rather than the hyperlocalized areas astronauts will target on individual spacewalks or surface excursions."

Lo que piden:
- "Create a map, tool, or interface a Mars astronaut could use during a surface exploration mission, using NASA data and tools."
- Empezar por definir objetivos científicos y el lugar a explorar. Ejemplo de la página: si el objetivo es estudiar formaciones *boxwork* (que Curiosity estudió en 2025), elegir un lugar donde abunden, como la región de Mount Sharp.
- Se puede partir de un lugar muy explorado por rovers u otras misiones (más datos e imágenes) o desafiarse con un área menos estudiada.
- "You may consider including at least three kinds of data" (óptico, térmico, espectral, topográfico, geofísico, atmosférico, radiación, químico, entre otros), superpuestos o combinados para dar contexto útil para navegar.
- La prioridad es la seguridad del astronauta: considerar qué información ayuda a moverse con seguridad por un ambiente nuevo, por ejemplo datos de clima espacial, porque el ambiente de radiación de Marte difiere del de la Tierra, la órbita terrestre y la Luna.
- Considerar que el astronauta lleva traje espacial y quizá equipo científico y suministros, y cómo eso afecta su movilidad; si es posible, incluir un mapa de ruta que muestre por dónde va a ir para cumplir los objetivos científicos evitando obstáculos importantes como grandes formaciones rocosas.

## Consideraciones potenciales
*Opcionales ("You may (but are not required to) consider the following").*

- El mapa puede pensarse como una pantalla dentro de la nave o el hábitat para planificar la ruta, una app de tablet que la tripulación lleva consigo, un HUD proyectado en el visor del casco u otro formato.
- Puede ser una imagen estática, un producto digital interactivo, o una herramienta o sistema que genere mapas de distintos lugares o con distintos tipos de datos.
- Inspirarse en cómo las misiones de NASA a otros destinos (Tierra, Luna, otros planetas, exoplanetas) han creado este tipo de mapas.
- Se pueden usar datos de un periodo pasado específico, datos o condiciones previstas para un periodo futuro, o ningún periodo en particular (por ejemplo si el foco es topografía y geología, que son más estables).
- Mirar las imágenes de las misiones de superficie de NASA para imaginar cómo sería ejecutar la misión con el mapa, e incluir esas imágenes en primera persona para que el astronauta sepa qué esperar al llegar.
- Pensar si la misión usaría rovers u otros sistemas robóticos para transportar a la tripulación y sus herramientas, o si los astronautas trabajarían en pareja para hacer más ciencia.

## Datos y recursos de NASA
*La pestaña Resources se publicó el viernes 13 de noviembre de 2026; antes decía "Resources will be available on Friday, November 13." Pueden agregarse más recursos.*

| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Jet Propulsion Laboratory's (JPL) Mars Trek | Portal web de NASA para explorar Marte; muestra datos recolectados por NASA en distintos sitios de aterrizaje. | Visor web (las capas también se exponen como servicios WMTS) | https://trek.nasa.gov/mars/ |
| USGS-NASA Planetary Geologic Mapping Program | Mapa geológico estandarizado de alta calidad de Marte. | Visor web (ArcGIS Hub) + descarga de mapas | https://astrogeology-usgs.hub.arcgis.com/pages/interactive-maps-mars |
| Example of a route map visual for the Curiosity example mentioned above | Animación con una ruta propuesta para Curiosity subiendo la parte baja de Mount Sharp; la versión anotada marca regiones que los científicos quieren explorar, y un video de sobrevuelo las explica. | Documento (imagen/video) | https://science.nasa.gov/resource/curiositys-proposed-path-up-mount-sharp/ |
| NASA Worldview | Exploración visual del pasado y presente de la Tierra desde la perspectiva de un satélite (parte de NASA Earthdata). Útil como referencia de cómo se arma un visor de capas. | Visor web | https://worldview.earthdata.nasa.gov/ |
| Mars Perseverance Raw Images Gallery | Más de 1 millón de imágenes capturadas por el rover en su recorrido por la superficie de Marte. | Visor web (galería; hay un feed JSON no documentado) | https://mars.nasa.gov/mars2020/multimedia/raw-images/ |

## Recursos de agencias socias
No especificado en la página oficial.

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Capas base sin descargar:** Mars Trek expone sus mosaicos como **WMTS** (p. ej. el mosaico CTX global, el color de Viking, el hillshade de MOLA o HRSC, y DEMs locales de HiRISE en algunos sitios). Se pueden consumir directamente desde MapLibre o CesiumJS como capa raster, sin servidor propio. Verifiquen la URL de cada capa en el panel "Layers → info" de Trek.
- **Topografía para rutas:** para el cálculo de pendientes usen un DEM. MOLA MEGDR (~463 m/px) alcanza para un contexto regional; para una ruta a escala de caminata necesitan DEMs **HiRISE** (1 m/px, disponibles solo en sitios puntuales como Gale/Mount Sharp y Jezero) o CTX (~6–20 m). Vienen en GeoTIFF/IMG desde el PDS o el sitio de HiRISE: recorten a unos pocos km².
- **Imágenes en primera persona:** la galería raw de Perseverance tiene un endpoint JSON (no documentado oficialmente; puede cambiar) para filtrar por sol y cámara. Alternativa estable: elegir a mano 10–20 imágenes e incluirlas como assets estáticos con su crédito NASA/JPL-Caltech.
- **Skills del repo que aplican:** space-apps-challenge, planetary-data, nasa-data-apis, maps-and-dataviz, frontend-design, ai-features (opcional), edu-games (si hacen un modo simulación), deploy-vercel-supabase, pitch-and-submission, hackathon-sprint.
