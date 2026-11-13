---
name: planetary-data
description: Herramientas para datos de la Luna, Marte y el cielo — Moon/Mars Trek, LROC Quickmap, PDS, JPL Horizons API y SPICE (efemérides, posición del Sol y la Tierra vista desde un punto de la superficie), sistemas de coordenadas planetarias, IRSA/astroquery/astropy para imágenes astronómicas (SPHEREx, WISE, Gaia) y Aladin Lite. Úsala cuando haya que obtener o mostrar datos de otros cuerpos del sistema solar o del cielo.
---

# Datos planetarios y astronómicos (herramientas)

> Herramientas y forma de acceso, no soluciones. El enfoque del challenge lo decide el equipo.

## Mapas de la Luna y Marte
- **Moon Trek / Mars Trek** (https://trek.nasa.gov/): mosaicos, DEM, pendiente e iluminación, con servicios WMTS para usar como capa base.
- **LROC Quickmap** (https://quickmap.lroc.asu.edu/): visor lunar muy completo (incluye las capas polares).
- **PDS** (https://pds.nasa.gov/): archivo primario de misiones. Pesado, úsalo cuando Trek no alcance.
- **Coordenadas:** muchos productos de Marte usan longitud este 0–360 (planetocéntrica). Normaliza a −180..180 antes de mapear. La Luna y Marte tienen radios y elipsoides propios: no uses el de la Tierra (WGS84).
- 3D: CesiumJS con `Ellipsoid.MOON` o un elipsoide marciano (ver `maps-and-dataviz`).

## Efemérides: JPL Horizons API (sin instalar nada)
Calcula la posición de un cuerpo vista desde un observador, incluso desde un punto de la superficie de otro cuerpo.
```
GET https://ssd.jpl.nasa.gov/api/horizons.api?format=json
  &COMMAND='10'                      # objetivo: 10 = Sol, 399 = Tierra, 499 = Marte, 301 = Luna
  &CENTER='coord@301'                # observador sobre la superficie de la Luna
  &COORD_TYPE='GEODETIC'&SITE_COORD='<lon_E>,<lat>,<alt_km>'
  &MAKE_EPHEM='YES'&EPHEM_TYPE='OBSERVER'
  &START_TIME='2027-01-01'&STOP_TIME='2027-01-31'&STEP_SIZE='6h'
  &QUANTITIES='4'                    # 4 = azimut/elevación aparentes
```
- La tabla viene en el campo `result`, entre `$$SOE` y `$$EOE`. Parsea y cachea: el resultado es determinista.
- Documentación: https://ssd-api.jpl.nasa.gov/doc/horizons.html
- La elevación que devuelve es **respecto del horizonte geométrico**. No considera el relieve local.

## SPICE (más control, offline)
- `pip install spiceypy`. Kernels genéricos: https://naif.jpl.nasa.gov/pub/naif/generic_kernels/ (lsk, pck y spk de440; frames lunares `moon_pa`/`moon_me`).
- Útil para muchas consultas de geometría o cuando la API limita la cantidad de pedidos.

## Imágenes astronómicas
- **IRSA** (https://irsa.ipac.caltech.edu/) aloja SPHEREx, WISE/NEOWISE y otros. Desde Python: `pip install astroquery astropy`, módulo `astroquery.ipac.irsa`. IRSA tiene un servicio de recortes (cutouts).
- **Gaia** (posiciones y movimientos propios): `astroquery.gaia`.
- **FITS → web:** abre con `astropy.io.fits`, alinea por WCS (`astropy.wcs`, `reproject`), escala con `astropy.visualization` (asinh, ZScale) y exporta PNG. No sirvas FITS crudos al navegador.
- **Aladin Lite** (https://aladin.cds.unistra.fr/AladinLite/): visor de cielo embebible con un `<script>`, compatible con HiPS.

## Imágenes y fichas de misiones
- NASA Image and Video Library API (sin key): `https://images-api.nasa.gov/search?q=<consulta>&media_type=image`
- NSSDCA Master Catalog (fichas técnicas de naves y misiones): https://nssdc.gsfc.nasa.gov/nmc/
- Raw images de rovers: las páginas de cada misión en https://science.nasa.gov/ . Acredita "NASA/JPL-Caltech" (y el instituto del instrumento si corresponde).
