# The Earth Information Jukebox
> La rockola de información de la Tierra

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/the-earth-information-jukebox/
- **Dificultad:** Intermediate, Beginner/Youth (Intermedio, Principiante/Juvenil)
- **Temas:** Arts & Multimedia, Earth Science, Software

## Resumen
El Earth Information Center (EIC) de NASA produce visualizaciones impactantes de nuestro planeta en cambio, pero presentar la ciencia de la Tierra solo mediante visualización limita a quién puede llegar. Se invita a hacer esta ciencia compleja accesible, atractiva y multisensorial traduciendo la vista en sonido. El desafío es construir un "Earth Jukebox": una interfaz, script o aplicación que combine fotogramas visuales del EIC con sonificaciones dinámicas generadas en tiempo real.

## Objetivos / qué piden
Contexto (Background): las visualizaciones del EIC (temperaturas globales, corrientes oceánicas, etc.) se presentan casi exclusivamente en formato visual, lo que limita la accesibilidad para personas ciegas o con baja visión. La sonificación (traducir datos a sonido) puede revelar patrones sutiles y generar una conexión emocional más profunda con el planeta. Ejemplos sugeridos: usar visión por computadora para mapear espectros de color a acordes musicales, traducir densidad de píxeles a ritmo, o reglas artísticas que controlen parámetros de sintetizador según formas visuales.

Lo que piden:

- "Create an 'Earth Jukebox' experience that generates dynamic sonifications from NASA's EIC visual frames."
- Resultado deseado: que las personas puedan seleccionar o experimentar una visualización específica del EIC junto con su paisaje sonoro (soundscape) correspondiente.
- Dos caminos válidos: una herramienta interactiva que traduzca activamente lo visual en sonido, **o** una colección curada de archivos de audio originales generados que cuenten la historia auditiva de los datos.
- Métodos libres: visión por computadora (paletas de color, formas, densidad de píxeles) mapeada a parámetros de audio (pitch, tempo, instrumentación); machine learning; frameworks open source de síntesis de audio; reglas artísticas basadas en datos.
- Apuntar a soundscapes que equilibren "data interpretation with artistic, musically pleasing outputs".
- Formatos de entrega posibles: jukebox web interactivo, app de escritorio, endpoint de API backend, o una playlist de audios generados acompañada de una explicación del proceso creativo.

## Consideraciones potenciales
*Opcionales, según la página oficial:*

**Entender los conceptos**
- **Sonificación:** uso de audio no verbal para transmitir información o percibir datos.
- **Fotogramas visuales:** muchas visualizaciones del EIC son secuencias animadas o imágenes; se pueden extraer fotogramas individuales como base de la generación de audio.

**Diseñar la experiencia "Jukebox"**
- La entrega puede ser desde una UI interactiva programada hasta una galería o playlist curada de audios generados junto con las imágenes.
- Si es interactiva, cuidar la UX: qué tan intuitivo es elegir una visualización e iniciar el audio.
- La accesibilidad es un tema central: compatibilidad con lectores de pantalla, feedback táctil o textos descriptivos ricos que acompañen el audio.

**Enfoques de traducción de datos**
- Mapear elementos visuales (espectros de color, brillo, formas geométricas, densidad espacial de píxeles) a parámetros de audio (pitch, ritmo, tempo, volumen, instrumentación).
- Balance entre traducción estricta y musicalidad: una traducción directa píxel-a-sonido puede sonar a ruido caótico; aplicar reglas artísticas, escalas musicales o armonías.
- Usar disciplinas variadas: algoritmos de visión por computadora, modelos de ML, sintetizadores y estaciones de trabajo de audio digital (DAW).

**Contar la historia**
- Documentar el proceso: el "por qué" de las decisiones artísticas y los algoritmos/scripts usados.
- Pensar en el impacto emocional: cómo los soundscapes pueden reflejar la urgencia del cambio climático o la belleza de los sistemas interconectados del planeta.

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Earth Information Center (EIC) Content Database | Conjunto de visuales del Scientific Visualization Studio (SVS) usados en los dashboards del EIC; videos e imágenes utilizables como datos para generar audio | Visor web + descarga directa (galería SVS) | https://svs.gsfc.nasa.gov/gallery/earth-information-center/ |
| Data Sonification: Caldwell 73 | Ejemplo de NASA que convierte imágenes astronómicas en sonido; marco conceptual para traducir luminosidad y coordenadas espaciales a volumen y pitch | Descarga directa (video/audio SVS) | https://svs.gsfc.nasa.gov/13786/ |
| Change in Night Lights between 2012 and 2023 – EIC Version | Evolución de la iluminación urbana global; alto contraste ideal para mapear intensidad a tempo o volumen | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/5313/ |
| Global Sea Surface Currents and Temperature | Animaciones de corrientes oceánicas y gradientes térmicos; para convertir movimiento continuo en soundscapes evolutivos | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/3912/#media_group_349716 |
| GEOS Aerosols | Trayectorias de humo, polvo y otras partículas atmosféricas por tipo; apto para experimentar con múltiples pistas/instrumentos | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/5669/ |
| Air Quality Model Runs | Frames descargables de alta resolución de modelos atmosféricos; para convertir paletas de color y densidad de píxeles en perfiles sonoros | Descarga directa (frames SVS) | https://svs.gsfc.nasa.gov/5533/ |
| Perpetual Ocean 2: Western Boundary Currents | Mapas visuales de las principales corrientes de borde occidental; alto contraste útil para visión por computadora y ritmos/escalas | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/5425/ |
| DYAMOND Global Carbon Dioxide | Concentraciones globales de CO2 con patrones cambiantes en el tiempo; input principal posible para síntesis interactiva | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/5196/ |
| Lightning Events Detected from the ISS 2017-2023 | Compilación de rayos detectados desde la ISS; ideal para sonificación percusiva con disparos ante cada destello | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/5011/ |
| A 3D View of an Atmospheric River from an Earth System Model | Visualización 3D de un río atmosférico; extraer frames para mapear densidad y extensión de humedad a paisajes musicales | Descarga directa (frames/video SVS) | https://svs.gsfc.nasa.gov/4960/ |
| GEDI Forest Height | Mapas globales de altura del dosel forestal; espectros de color (alturas/densidades) mapeables a ritmo, frecuencia o pitch | Descarga directa (imágenes/video SVS) | https://svs.gsfc.nasa.gov/4950/ |

## Recursos de agencias socias
No especificado en la página oficial.

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Cómo obtener los frames:** cada página SVS (`svs.gsfc.nasa.gov/<id>/`) lista medios descargables (MP4/MOV en varias resoluciones y, en muchas, carpetas de frames PNG/TIFF numerados). Sin autenticación. SVS también expone una API JSON por visualización (sugerencia adicional: `https://svs.gsfc.nasa.gov/api/<id>`) útil para listar URLs de medios automáticamente.
- **Gotcha de tamaño:** las secuencias de frames 4K pueden pesar varios GB. Para el MVP, bajar el MP4 de 1080p y extraer frames con `ffmpeg -i in.mp4 -vf fps=2,scale=480:-1 frames/%04d.png`, o precomputar directamente las features por frame (brillo medio, histograma de tono HSV, % de píxeles sobre umbral, centroide, flujo óptico) en un notebook y guardarlas en JSON de pocos KB.
- **Skills del repo aplicables:** `space-apps-challenge`, `earth-geospatial-data`, `nasa-data-apis`, `frontend-design`, `maps-and-dataviz`, `ai-features` (si usan ML para mapear features a música), `deploy-vercel-supabase`, `hackathon-sprint`, `pitch-and-submission`.
