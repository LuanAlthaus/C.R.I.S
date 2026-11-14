# Field Shift: Adapting Farms with NASA Data
> Field Shift: adaptar los campos con datos de NASA

- **URL oficial:** https://www.spaceappschallenge.org/2026/challenges/adapting-farms-with-nasa-data/
- **Dificultad:** Advanced, Intermediate
- **Temas:** Arts & Multimedia, Earth Science, Software

## Resumen
Los agricultores de todo el mundo lidian con temperaturas cambiantes, patrones de lluvia alterados, escasez de agua, clima extremo y deterioro de la salud del suelo. Estas presiones hacen difícil elegir rotaciones de cultivos que protejan los campos, conserven el agua y sostengan la resiliencia a largo plazo. Tu desafío es crear una herramienta de apoyo a la decisión que use observaciones de la Tierra de NASA junto con información local de suelos, características de los cultivos y prioridades del agricultor, para ayudar a explorar estrategias de rotación que puedan fortalecer la salud del suelo y adaptar los campos a condiciones cambiantes.

## Objetivos / qué piden
Contexto (Background): elegir cultivos nuevos para una rotación es difícil porque un cultivo que mejora el suelo en un lugar puede consumir demasiada agua, introducir enfermedades, chocar con los mercados locales o quedar fuera de la temporada de crecimiento en otro. El desafío se inspira en parte en la experiencia de familias agricultoras de varias generaciones; los datos ambientales de largo plazo de NASA y las técnicas de SI (*super intelligence*, el término que Space Apps 2026 usa en lugar de "AI") abren la posibilidad de traducir esos datos en información práctica para el agricultor.

Lo que plantea la página:
- Combinar observaciones de la Tierra de NASA con información agrícola local para crear "an app, model, interactive map, or other decision‑support tool" que ayude a explorar opciones de rotación que mejoren la salud del suelo y la resiliencia frente al clima y el agua.
- Datasets de NASA sugeridos: SMAP, ECOSTRESS, GPM, Landsat, MODIS, NASA POWER, GRACE y GRACE‑FO, y SRTM u otros productos de elevación.
- Analizar patrones de temperatura, estrés térmico, lluvia, humedad del suelo, evaporación, riesgo de helada, precipitación extrema, estrés de vegetación y tendencias hídricas de largo plazo.
- Incorporar suelo local, características de cultivos y prioridades del agricultor: uso de agua, valor forrajero, fijación de nitrógeno, reducción de erosión, mejora de materia orgánica o adecuación al mercado.
- La herramienta puede mostrar cuándo los campos suelen quedar desnudos, en qué temporadas los cultivos actuales son más vulnerables, u oportunidades de rotación para cortar ciclos de plagas, bajar la demanda de agua, mejorar la infiltración o sumar ingresos.
- "You may compare at least three rotation strategies, including the existing rotation." Para cada estrategia: por qué se sugirió, qué evidencia la respalda, qué beneficios puede traer, dónde hay vacíos de información y qué riesgos requieren revisión local por agricultores o agrónomos.
- Resultado esperado: "a supportive tool", que ayude a identificar patrones y posibilidades "but does not replace farmer judgment or local agricultural expertise".

## Consideraciones potenciales
*Opcionales ("You may (but are not required to) consider the following").*

**Comprender condiciones y datos**
- Cómo se relacionan los cambios de temperatura, humedad del suelo, lluvia, evaporación y estrés de vegetación con la vulnerabilidad del cultivo y la salud del suelo.
- Explicar términos como evapotranspiración, anomalías de humedad del suelo, índices de vegetación (p. ej. NDVI), riesgo de helada e indicadores de balance hídrico.
- Aclarar en qué difieren las observaciones satelitales de las mediciones en terreno, y cómo combinarlas mejora la precisión.

**SI, modelado y fusión de datos**
- Comparar distintos enfoques de SI o estadísticos para encontrar patrones en el historial de clima y suelo.
- Usar SI para resaltar transiciones estacionales, periodos de campo desnudo o momentos de mayor estrés del cultivo.
- Explicar cómo el modelo interpreta la incertidumbre y dónde hace falta validación local experta.
- Armonizar datasets con distinta resolución espacial y temporal (submuestreo, agregación o remuestreo a una grilla común) para combinar humedad del suelo SMAP, temperatura superficial ECOSTRESS, precipitación GPM, índices de vegetación Landsat/MODIS, biomasa GEDI y datos de suelo en variables coherentes y explicables.

**Exploración de estrategias de rotación**
- Comparar escenarios según ciclo de nutrientes, riesgo de erosión, aporte de materia orgánica, corte de ciclos de plagas, uso de agua, potencial forrajero o mercado.
- Mapear qué ideas de rotación se alinean mejor con las prioridades del agricultor (conservar agua, ingresos extra, mejorar estructura del suelo).
- Explicar brevemente cómo aportan a la salud del suelo los cultivos fijadores de nitrógeno, de cobertura, de raíz profunda o tolerantes a sequía.

**Mapas y visualización geoespacial**
- Mapas interactivos para explorar tendencias (humedad del suelo de largo plazo, olas de calor, caída del almacenamiento de agua).
- Visualizar vulnerabilidades a nivel de lote (zonas de erosión, focos de estrés hídrico, áreas propensas a helada).
- Extras opcionales: líneas de tiempo, capas superpuestas que comparan datasets, o sliders de escenarios que ajustan lluvia o temperatura.

**Diseño centrado en el agricultor**
- Que sea fácil de interpretar para agricultores con poca experiencia técnica o geoespacial.
- Permitir que el agricultor ingrese sus prioridades o restricciones (agua disponible, maquinaria, preferencias de mercado).
- Recordar que la herramienta apoya, no reemplaza, el conocimiento local, y señalar dónde el juicio humano es especialmente importante.

## Datos y recursos de NASA
Recursos de NASA listados en la sección "NASA Data & Resources" de la página:

| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Soil Moisture Active Passive (SMAP) | Humedad superficial del suelo y estado congelado/descongelado a escala global, cada 2–3 días. Útil para estrés por sequía, disponibilidad de agua e infiltración. | Dataset en catálogo (Earthdata; descarga requiere Earthdata Login) | https://www.earthdata.nasa.gov/data/platforms/space-based-platforms/smap |
| Global Precipitation Measurement (GPM) & IMERG | Tasas y acumulados de precipitación global. Sirve para confiabilidad de lluvias, tormentas extremas, riesgo de erosión y tendencias. La página indica que las capas IMERG de Worldview se pueden descargar como imágenes de alta resolución. | Dataset en catálogo (descarga requiere Earthdata Login) | https://gpm.nasa.gov/data |
| Landsat | Reflectancia, NDVI e índices de vegetación a 30 m: vigor del cultivo, suelo desnudo, transiciones estacionales. | Descarga directa vía USGS EarthExplorer (requiere cuenta USGS) | https://earthexplorer.usgs.gov/ |
| MODIS | Reflectancia e índices de vegetación (250–1000 m) del catálogo de productos LP DAAC. | Dataset en catálogo (requiere Earthdata Login) | https://lpdaac.usgs.gov/products/ |
| VIIRS | Reflectancia e índices de vegetación (250–1000 m); el enlace es una búsqueda en Earthdata Search. | Dataset en catálogo (requiere Earthdata Login) | https://search.earthdata.nasa.gov/search?q=VIIRS |
| NASA POWER | Datos agroclimáticos y meteorológicos: temperatura, humedad, radiación e indicadores derivados (uso de agua del cultivo, riesgo de helada, aptitud climática). | API REST (sin login) | https://power.larc.nasa.gov/ |
| Shuttle Radar Topography Mission (SRTM) | Elevación global para pendiente, escorrentía, erosión y flujo de agua. | Dataset en catálogo (requiere Earthdata Login) | https://www.earthdata.nasa.gov/data/instruments/srtm |
| ECOSTRESS | Temperatura superficial (LST) y evapotranspiración (ET) en alta resolución: focos de estrés térmico y demanda hídrica a nivel de lote. | Dataset en catálogo (requiere Earthdata Login) | https://ecostress.jpl.nasa.gov/data |
| NLDAS/GLDAS: Land Data Assimilation Systems | Grillas sub-diarias de temperatura, precipitación, humedad del suelo, proxies de ET y radiación. La página indica buscar "NLDAS" o "GLDAS" dentro de Earthdata Search. | Dataset en catálogo (requiere Earthdata Login) | https://www.earthdata.nasa.gov/ |
| Global Ecosystem Dynamics Investigation (GEDI) | Lidar de estructura de vegetación: altura de dosel, índice de área foliar, biomasa aérea (productos Level‑4). | Dataset en catálogo (requiere Earthdata Login) | https://gedi.umd.edu/data |

Nota: la página también nombra GRACE y GRACE‑FO en los objetivos, pero no incluye un enlace para ellos en la pestaña Resources.

## Recursos de agencias socias
**ESA (European Space Agency)**

| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| Copernicus Sentinel-2: ESA | Imágenes multiespectrales libres a 10–20 m con alta frecuencia de revisita; bandas red‑edge, NIR y SWIR para NDVI/EVI, suelo desnudo y estrés de cultivo. | Descarga directa / API (Copernicus Data Space; requiere cuenta gratuita) | https://dataspace.copernicus.eu/ |
| ERA5: ECMWF Reanalysis | Reanálisis global con variables horarias: temperatura, precipitación, viento, humedad, temperatura del suelo, radiación. | API REST (Climate Data Store; requiere cuenta gratuita) | https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=overview |

## Otros recursos
La página lista estos recursos de otras instituciones dentro de la misma sección "NASA Data & Resources" (no son de NASA):

| Recurso | Qué es | Tipo de acceso | URL |
|---|---|---|---|
| USDA Ag Data Commons | Datasets agrícolas abiertos: suelos, cultivos, clima, manejo, fenología, calidad de agua. | Dataset en catálogo | https://www.nal.usda.gov/services/agdatacommons |
| Web Soil Survey (WSS) | Datos oficiales de suelos de EE. UU. (USDA NRCS): mapas, propiedades, drenaje, erosión, aptitud. | Visor web | https://websoilsurvey.nrcs.usda.gov/app/ |
| Cropland Data Layer (CDL): CropScape | Clasificación anual de cultivos a 30 m para EE. UU. (maíz, soja, algodón, trigo, alfalfa, etc.). | Visor web + descarga | https://nassgeodata.gmu.edu/CropScape/ |
| PRISM Climate Data (Oregon State University) | Clima de EE. UU. diario, mensual y normales de 30 años, con efecto topográfico. | Descarga directa | https://prism.oregonstate.edu/ |
| USDA NRCS SSURGO/STATSGO2 | Atributos de suelo a escala de lote (SSURGO) y regional (STATSGO2). | Descarga directa | https://www.nrcs.usda.gov/resources/data-and-reports/soils |
| National Agriculture Imagery Program (NAIP) | Imágenes aéreas de ~1 m durante la temporada de cultivo en EE. UU. | Visor web + descarga | https://naip-usdaonline.hub.arcgis.com/ |
| USGS National Land Cover Database (NLCD) | Cobertura y uso del suelo a 30 m en varias décadas. | Descarga directa | https://www.usgs.gov/centers/eros/science/national-land-cover-database |
| FAO Global Agro-Ecological Zones (GAEZ) | Aptitud agroclimática global por cultivo, restricciones de suelo, rendimientos potenciales. | Visor web + descarga | https://gaez.fao.org/ |
| CoCoRaHS | Mediciones colaborativas diarias de lluvia/nieve en Norteamérica, útiles para validar IMERG. | Visor web | https://www.cocorahs.org/Maps |
| USGS National Water Information Systems (NWIS) | Niveles freáticos, caudales y calidad de agua de largo plazo. | Visor web + API | https://waterdata.usgs.gov/nwis |
| USDA AMS Market News | Precios de commodities y condiciones de mercado de cultivos y ganado. | Documento / reportes web | https://www.ams.usda.gov/market-news |
| USDA NASS QuickStats | Estadísticas agrícolas por condado y estado: rendimientos, superficie, producción. | Visor web + API | https://quickstats.nass.usda.gov/ |
| OpenET | Evapotranspiración a escala de lote para estados del oeste de EE. UU. | Visor web + API | https://openetdata.org/ |
| USFS LANDFIRE | Tipo y condición de vegetación, modelos de combustible, características del paisaje en EE. UU. | Descarga directa | https://www.landfire.gov/ |

## Acceso a los datos (notas del repo)

> Por la regla oficial, no se puede trabajar en un challenge antes del hackathon, ni usar SI (IA) para idear la solución. Por eso esta sección solo trae notas de acceso a los datos, no enfoques de solución. La idea la pone el equipo.

*No es parte de la página oficial. Las URLs y endpoints que no están en la página son sugerencias adicionales: verificalos.*

- **Empezar por NASA POWER:** su API (`/api/temporal/daily/point`, comunidad `AG`) devuelve JSON/CSV por punto lat/lon sin autenticación; con eso solo ya se obtienen temperatura, precipitación, radiación, humedad y días de helada para 40+ años. Es la base más rápida para un MVP. *(Sugerencia adicional: endpoint concreto no listado en la página.)*
- **Earthdata Login** es obligatorio para SMAP, ECOSTRESS, GPM IMERG, MODIS/VIIRS, SRTM, GLDAS y GEDI. Creen la cuenta antes del hackathon y usen `earthaccess` (Python) para buscar y bajar granulos. Formatos: SMAP y GEDI en HDF5, IMERG y GLDAS en HDF5/NetCDF4, ECOSTRESS en HDF5 o COG, MODIS en HDF4/HDF-EOS. *(Sugerencia adicional.)*
- **Cuidado con el tamaño:** no descarguen globales. Recorten por bounding box del lote o del condado y agreguen a series mensuales en un notebook (xarray + rioxarray). Para el front, exporten solo JSON chico por lote (series de tiempo agregadas) en lugar de rásteres.
- **Skills del repo que aplican:** space-apps-challenge, nasa-data-apis, earth-geospatial-data, maps-and-dataviz, ai-features, frontend-design, deploy-vercel-supabase, pitch-and-submission, hackathon-sprint (data-biz si quieren armar el caso de uso o negocio para cooperativas).
