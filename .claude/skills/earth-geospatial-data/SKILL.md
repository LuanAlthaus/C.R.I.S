---
name: earth-geospatial-data
description: Técnicas para datos de observación de la Tierra (MODIS, VIIRS, SAR/NISAR/Sentinel-1, SMAP, Landsat, IMERG, MERRA-2, POWER) — formatos HDF5/NetCDF/GeoTIFF/COG, descubrimiento y descarga con Earthdata, recortes, reproyección, series temporales, tests de tendencia y cómo servir rasters livianos a una web app. Úsala cuando haya que convertir datos satelitales de la Tierra en algo que la app pueda mostrar.
---

# Datos geoespaciales de la Tierra (técnicas genéricas)

> Esta skill trae **herramientas y técnicas**, no soluciones. La idea y el enfoque para resolver el challenge los decide el equipo. Por la regla oficial, la IA no puede idear la solución.

## Pipeline estándar
1. **Acotar área y período:** una región y unos pocos años. Nada global salvo que sea imprescindible.
2. **Descubrir:** CMR o Earthdata Search (https://search.earthdata.nasa.gov/) → `short_name` y versión de la colección.
3. **Descargar recortado:** AppEEARS (recorta y entrega GeoTIFF/CSV), después `earthaccess` con subset, y en último caso archivos completos.
4. **Procesar en un notebook** (`notebooks/`): `xarray` + `rioxarray` para rasters, `geopandas` para vectores. Para tablas grandes (millones de filas CSV), DuckDB.
5. **Exportar liviano:** GeoJSON (menos de 5 MB), COG con overviews, PNG georreferenciado, PMTiles, Parquet o una tabla en Supabase/PostGIS.
6. **Servir:** archivos estáticos en `public/data/` (los cachea la CDN de Vercel). Lo dinámico, en Supabase.

```python
import xarray as xr, rioxarray  # noqa
ds = xr.open_dataset("data/raw/file.nc")        # NetCDF / HDF5 (engine="h5netcdf" si hace falta)
da = ds["variable"].rio.write_crs("EPSG:4326")
clip = da.rio.clip_box(minx=-62, miny=-28, maxx=-54, maxy=-19)
clip.rio.to_raster("public/data/var.tif", driver="COG")
```

## Formatos
| Formato | Abrir con | Nota |
|---|---|---|
| NetCDF / HDF5 | `xarray` (`h5netcdf`, `netCDF4`) | Revisa `scale_factor`, `_FillValue` y unidades en los atributos |
| HDF4 (MODIS viejo) | `pyhdf` o `rioxarray` con GDAL compilado con HDF4 | O pídelo ya convertido en AppEEARS |
| GeoTIFF / COG | `rioxarray`, `rasterio` | COG = se lee por rangos HTTP, ideal para web |
| CSV de puntos | `pandas` / `duckdb` / `geopandas` | Fechas en UTC |
| Zarr | `xarray.open_zarr` | Datos en la nube (por ejemplo, en S3) |

## Series temporales
- Agrega a resolución mensual o anual y calcula **anomalías** respecto de la climatología (media por mes del calendario).
- Tests de tendencia monótona: **Mann-Kendall** (`pip install pymannkendall`) con **Sen's slope** para la magnitud. Reporta el p-valor.
- Muchas pruebas a la vez (por píxel): corrige por comparaciones múltiples (Benjamini-Hochberg) o explica por qué no lo hacés.
- Autocorrelación: las series climáticas la tienen. Usa la variante de Mann-Kendall con pre-whitening (Hamed-Rao) si aplica.

## Combinar sensores distintos
Antes de comparar o unir series de sensores distintos, documenta su **resolución espacial, período de operación, horario de pasada y sensibilidad**. Cambiar de sensor puede crear una "tendencia" falsa. Compara siempre en el período en que ambos se solapan y explica en la UI qué decisión tomó el equipo.

## SAR (radar)
- Datos de NISAR y Sentinel-1 en ASF DAAC: https://search.asf.alaska.edu/ ; en Python, `pip install asf_search`. Requiere Earthdata Login.
- Conceptos que hay que explicar al público: retrodispersión (backscatter, en dB), polarización (VV/VH/HH/HV), coherencia, interferometría (InSAR).
- Las escenas pesan GB: elige pocas y pre-procesa a PNG o COG.
- Si un producto no cubre el sitio o la fecha que buscas, revisa la cobertura en ASF Vertex antes de comprometerte.

## Visualización
- Escala secuencial para magnitudes y divergente centrada en 0 para anomalías. La leyenda lleva unidades.
- Fondo de mapa: teselas GIBS (ver `nasa-data-apis`) o un estilo vectorial libre en MapLibre. Ver `maps-and-dataviz`.

## Checklist de rigor (criterio "Validity" del jurado)
- [ ] Fuente, producto, versión y resolución de cada dato, visibles en la UI ("Datos" o "Acerca de").
- [ ] Período y área analizados.
- [ ] Limitaciones: nubes, cambio de sensor, huecos.
- [ ] Unidades en ejes y leyendas.
