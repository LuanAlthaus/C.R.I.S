# Be An Earth System Trend Detective!
> ¡Sé un detective de tendencias del sistema Tierra!

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/earth-system-trend-detective/
- **Dificultad:** Advanced
- **Temas:** Earth Science, Software

## Resumen
El sistema ambiental interconectado de la Tierra cambia constantemente, y cuando ese cambio sigue una dirección consistente (sube o baja, aumenta o disminuye, se engrosa o se adelgaza) las variables medidas lo reflejan. Pero una variable de ciencias de la Tierra puede tener una tendencia en una región y la opuesta en otra, aun cuando el mismo proceso impulsa ambas. La gran cantidad de variables hace difícil elegir cuáles podrían indicar cambio y luego analizarlas. El desafío es crear una herramienta que permita seleccionar variables medidas por misiones de ciencias de la Tierra o producidas por modelos, visualizar cómo cambian en el tiempo y determinar qué está cambiando, dónde, cuánto, y si esos cambios son lo que los científicos llaman "significativos".

## Objetivos / qué piden
Contexto (Background):
- Instrumentos satelitales observan tierra, atmósfera y océanos; los modelos globales combinan datos satelitales y de otro tipo. Un uso clave es caracterizar cambios, sobre todo tendencias.
- Dos partes del problema: **encontrar variables con tendencia** (desde temperatura y lluvia hasta evaporación o caudal) y **analizar tendencias en regiones específicas** de tierra, océano o niveles de la atmósfera.
- Instrumentos citados: MODIS, OMI, AIRS, GPM. Modelos citados: MERRA-2 y GLDAS.

Objetivos (una respuesta exitosa integra cuatro elementos):
1. **Identificar variables de ciencias de la Tierra de interés** que puedan investigarse por posibles tendencias.
2. **Adquirir los datos de la variable para una región particular** ("most trends in the data will not be global – though some trends can be!").
3. **Generar y visualizar una serie temporal** para la variable y región, "over a long enough period of time to allow statistical analysis".
4. **Realizar un análisis estadístico básico** de la serie, indicando la tasa de cambio en el tiempo, y **probar la significancia de la tendencia** con métodos como "a t-test, the Mann-Kendall test, or confidence intervals".
- "A response that does both the visualization and the analysis meets the full scope of this challenge."

## Consideraciones potenciales
**Audiencia**
- Estudiantes de secundaria y universidad, y público con algo de formación científica.
- Dar definiciones simples de las variables ayudaría mucho.
- La idea es que los usuarios se vuelvan "data detectives": encontrar variables, ver si tienen tendencia y si es significativa.

**Enfoque**
- Los usuarios podrían responder una breve serie de preguntas sobre qué o dónde quieren investigar, y las respuestas guiarían la búsqueda de variables. Sería muy útil incluir un par de ejemplos de demostración.
- Aunque no es obligatorio, "SI" (sic en la página; probablemente IA) podría servir para identificar datos con potencial de mostrar tendencias en ciertas zonas, a partir de noticias, papers o características de la región (desierto, selva, estuario, atmósfera sobre una gran ciudad).

**Trabajo con los datos**
- Hay datos horarios, cada 3 horas, diarios y mensuales; **diarios y mensuales son probablemente los más fáciles**.
- Encontrar una tendencia es interesante aunque no sea significativa; a veces solo hace falta un período más largo.
- Considerar herramientas públicas de graficado hidrológico de organizaciones externas como CUAHSI.
- Las tendencias pueden ser no lineales y algunas solo aparecen al remover ciclos (como el estacional). Análisis más complejos están fuera del alcance: **"detecting and analyzing linear trends would be a great start."**

## Datos y recursos de NASA
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Giovanni | Herramienta con menús de numerosas variables; genera series temporales y permite descargar sus datos | Visor web (requiere Earthdata Login para descargar) | https://giovanni.gsfc.nasa.gov/ |
| Hydrology Time Series | Genera series temporales con datos hidrológicos (GES DISC) | Visor web / API REST (requiere Earthdata Login) | https://disc.gsfc.nasa.gov/information/tools?title=Hydrology%20Time%20Series |
| AppEEARS | Acceso y transformación simple de datos geoespaciales de varios archivos federales; incluye generación de series temporales | Visor web + API REST (requiere Earthdata Login) | https://appeears.earthdatacloud.nasa.gov/ |
| Climate Data Online (CDO) | Acceso gratuito al archivo de NCDC (NOAA) de datos históricos globales de clima y tiempo, con historial de estaciones y herramienta de mapas. Nota: es un recurso de NOAA, listado en la sección NASA | Visor web / descarga directa (API con token gratuito) | https://www.ncei.noaa.gov/cdo-web/ |
| Time-Series Analysis of Remotely-Sensed SeaWiFS Chlorophyll in River-Influenced Coastal Regions | Artículo de investigación que demuestra creación de series temporales y prueba de significancia | Documento (PDF, NTRS) | https://ntrs.nasa.gov/api/citations/20090042747/downloads/20090042747.pdf |

## Recursos de agencias socias
| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| CSA: Radarsat Constellation Mission (RCM) - Data | Trío de satélites canadienses de observación terrestre (día/noche, todo clima) para vigilancia marítima, gestión de desastres y monitoreo de ecosistemas | Dataset en catálogo | https://donnees-data.asc-csa.gc.ca/en/dataset/rcm |
| CSA: Geo.ca | Fuente definitiva de información geoespacial abierta de Canadá; útil para temas ambientales en Canadá | Visor web / dataset en catálogo | http://geo.ca/ |
| CSA: SWOT Mission - Information | Misión NASA-CNES (con CSA y UK Space Agency) que mide cambios en la elevación de la superficie del agua de lagos, ríos, embalses y océanos | Documento / página informativa | https://www.asc-csa.gc.ca/eng/satellites/swot/ |
| CSA: Monitoring and protecting our ecosystem from space | Cómo los datos satelitales ayudan a monitorear cambios y usar responsablemente la tierra y recursos naturales | Documento / página informativa | https://www.asc-csa.gc.ca/eng/satellites/everyday-lives/monitoring-and-protecting-our-ecosystems-from-space.asp |
| CSA: Synthetic Aperture Radar python Tutorial | Tutorial para trabajar imágenes SAR con Python y QGIS, con pasos prácticos de acceso y visualización | Repositorio GitHub | https://github.com/asc-csa/Synthetic_Aperture_Radar_Tutorial |

## Otros recursos
La página lista estos bajo el encabezado "Space Agency Partner Resources", pero no son agencias espaciales:

| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Royal Netherlands Meteorological Institute (KNMI) Climate Explorer | Herramienta web de análisis científico del KNMI para explorar, analizar y visualizar datos climáticos de muchas fuentes | Visor web | https://climexp.knmi.nl/ |
| Statistical test methods to detect trends in time series data | Artículo sobre pruebas de tendencias en datos | Documento (Medium) | https://medium.com/@snp.kriss/statistical-test-methods-to-detect-trends-in-time-series-data-b361a0ec530b |
| A Comprehensive Guide to Statistical Significance | Guía sobre pruebas de significancia | Documento | https://www.statsig.com/perspectives/a-comprehensive-guide-to-statistical-significance |

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Acceso programático:** el camino más rápido es Giovanni o la Hydrology Time Series (GES DISC), que devuelven series ya promediadas por punto o región en CSV, evitando bajar grillas completas. AppEEARS tiene API REST (token vía Earthdata Login) para extraer series por punto/polígono de MODIS, etc. Todos requieren una cuenta gratuita de **Earthdata Login**; créenla antes del evento.
- **Formatos:** MERRA-2, GLDAS, GPM IMERG y AIRS vienen en NetCDF4/HDF5; MODIS en HDF-EOS/HDF4 o COG en la nube. Para grillas usar `xarray` + `netCDF4`/`h5netcdf`; sugerencia adicional: `earthaccess` (Python) simplifica la autenticación y descarga, y OPeNDAP permite recortar por región antes de bajar.
- **Cuidado con el tamaño:** un año de MERRA-2 horario global son decenas de GB. Usar productos **mensuales** (p. ej. M2TMNXSLV, GLDAS_NOAH025_M) y recortar la región en el servidor. Precalcular 5-10 series de demo y guardarlas como JSON/CSV.
- **Skills del repo aplicables:** space-apps-challenge, earth-geospatial-data, nasa-data-apis, maps-and-dataviz, ai-features, frontend-design, deploy-vercel-supabase, pitch-and-submission, hackathon-sprint.
