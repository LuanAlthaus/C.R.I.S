# Harmonization of MODIS and VIIRS Hot Spots
> Armonización de focos de calor MODIS y VIIRS

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/modis-and-viirs-hot-spots/
- **Dificultad:** Advanced, Intermediate
- **Temas:** Earth Science, Software

## Resumen
Entender cómo se distribuye la actividad de quema en el planeta a lo largo del tiempo, incluidos incendios forestales y quemas agrícolas lo bastante intensos para que los detecten los sensores satelitales, permite anticipar periodos críticos, orientar el monitoreo y mejorar la respuesta a incendios. Los satélites registran focos de fuego activo desde hace más de dos décadas, pero ese registro está dividido entre sensores cuyos datos no se pueden comparar directamente, y los gestores de incendios no cuentan con una imagen consistente de cuándo y dónde hubo quemas. Tu desafío es construir una aplicación web que armonice esos registros en un **calendario de actividad de quema** basado en focos de fuego activo, para que los equipos de alerta temprana y emergencia, científicos y gestores del territorio puedan examinar patrones históricos de fuego, condiciones inusuales y periodos críticos dentro de un área de interés seleccionada.

## Objetivos / qué piden
Contexto (Background):
- **Regímenes de fuego:** algunos ecosistemas necesitan fuegos periódicos, pero los cambios en intensidad, extensión y frecuencia afectan el clima, la sociedad, la biodiversidad y los servicios ecosistémicos, y pueden causar pérdidas de vidas y bienes. Los fuegos son cada vez más de origen humano (agricultura, desmonte, quema de residuos, actividad industrial).
- **Detección satelital:** los modelos climáticos proyectan más y más intenso "fire weather". Los sensores en infrarrojo térmico (TIR) e infrarrojo medio (MIR) detectan focos de calor (incendios, volcanes, quema de gas en pozos). MODIS y VIIRS dan detección de anomalías térmicas en tiempo casi real.
- **Calendarios de quema:** integrando historia y datos recientes se ven patrones estacionales, anomalías y tendencias por región.
- **El núcleo del problema:** MODIS (a bordo de Terra y Aqua) tiene más de dos décadas de datos pero se acerca al fin de su vida útil y está siendo reemplazado por VIIRS. "Their records cannot be directly compared without harmonization, since MODIS and VIIRS have different spatial resolutions, sensitivities, and observation schedules."
- Se sugiere armonizar los focos de NASA Earthdata FIRMS, combinando la larga serie histórica de MODIS con la continuidad y mayor detalle de VIIRS.

Lo que piden:
- "Develop a harmonized burning activity calendar based on active fire hotspots in a web application", combinando MODIS y VIIRS en una serie temporal consistente.
- Enfoque sugerido en dos partes:
  1. **Armonización MODIS–VIIRS:** al terminar esta fase "the sensor of origin no longer matters" y el foco pasa a patrones, tendencias y anomalías.
  2. **Aplicación web** que toma los datos armonizados: el usuario selecciona una región y un rango de fechas y descarga analíticas y visualizaciones de la actividad de fuego.
- Un calendario de quema con la cantidad de focos detectados desde principios de los 2000 hasta hoy, que muestre qué periodos del año tienen más fuego, qué años tuvieron más actividad y cómo se compara un fuego actual con los anteriores (p. ej. mes actual vs. mediana histórica del área).
- Usar rampas de color para representar similitudes y diferencias en la cantidad de focos entre meses.
- "A downloadable CSV File of hot spots detected during a search would be another useful output."

## Consideraciones potenciales
*Opcionales ("You may (but are not required to) consider the following").*

- Público objetivo: equipos de alerta temprana y emergencia, científicos, gestores del territorio y bomberos.
- Hacer la app interactiva: por ejemplo un mapa donde el usuario dibuje el área de interés, o la opción de ingresar latitud y longitud.
- Usar rampas de color accesibles.
- La herramienta podría mostrar un calendario de quema.
- Centrarse en pocos índices y figuras clave presentados con visualizaciones, como:
  - Comparaciones históricas, incluidas anomalías frente a promedios multianuales.
  - Cantidad de fuegos detectados y reportados.

Crédito de la foto de la página: Comisión Nacional de Actividades Espaciales de Argentina (CONAE).

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| FIRMS NASA EARTHDATA: Fire Information for Resource Management System | Acceso global en tiempo casi real a imágenes satelitales, focos de fuego activo y productos relacionados para ubicar, medir la extensión y la intensidad de incendios; datos geoespaciales, productos y servicios para la comunidad de manejo del fuego y el público. | API REST (FIRMS API con MAP_KEY gratuita) + visor web + descarga de archivo histórico | https://www.earthdata.nasa.gov/data/tools/firms |

## Recursos de agencias socias
No especificado en la página oficial. (La foto del desafío tiene crédito de CONAE, pero la página no lista recursos de CONAE.)

## Otros recursos
No especificado en la página oficial.

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Cómo bajar los datos:** FIRMS tiene (a) una **API de área** (`/api/area/csv/{MAP_KEY}/{SOURCE}/{bbox}/{días}/{fecha}`) con un MAP_KEY gratuito pedido por email, que devuelve CSV y sirve para datos recientes por bbox (límite de días por consulta y transacciones por cada 10 minutos), y (b) **"Archive Download"** en el sitio de FIRMS, donde se pide la serie histórica completa de un país o polígono (MODIS C6.1 desde nov-2000; VIIRS S-NPP desde ene-2012; NOAA-20 y NOAA-21 después) y llega por email en CSV, SHP o JSON. El pedido del archivo histórico puede demorar: háganlo apenas empiece el hackathon.
- **Fuentes (SOURCE):** `MODIS_SP`/`MODIS_NRT`, `VIIRS_SNPP_SP`/`_NRT`, `VIIRS_NOAA20_*`, `VIIRS_NOAA21_*`. SP = "standard processing" (calidad científica), NRT = tiempo casi real.
- **Tamaño:** el archivo global es de decenas de millones de filas; trabajen por país o región. Preprocesen en Python (pandas/DuckDB/GeoPandas) a una tabla agregada `celda, año, mes, conteo_armonizado` y súbanla a Supabase o como Parquet/JSON estático.
- **Skills del repo que aplican:** space-apps-challenge, nasa-data-apis, earth-geospatial-data, maps-and-dataviz, frontend-design, deploy-vercel-supabase, pitch-and-submission, hackathon-sprint (ai-features es opcional, por ejemplo para un resumen en texto de las anomalías).
