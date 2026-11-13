---
name: maps-and-dataviz
description: Elegir e implementar mapas y visualizaciones en la web app — MapLibre GL (2D), deck.gl (capas de datos masivos), CesiumJS (globos 3D de la Tierra, Luna y Marte), Three.js, Aladin Lite (cielo), y gráficos con Observable Plot, Recharts o ECharts. Incluye integración con Next.js (componentes cliente, carga dinámica), rendimiento y accesibilidad. Úsala cuando haya que mostrar datos espaciales o series temporales.
---

# Mapas y visualización de datos

## Elegir la herramienta
| Necesidad | Librería | Nota |
|---|---|---|
| Mapa 2D de la Tierra con capas raster/vector | **MapLibre GL JS** (`maplibre-gl`) | Gratis, sin token. Teselas GIBS como raster. |
| Muchos puntos (más de 10 000 focos), heatmaps, hexágonos | **deck.gl** sobre MapLibre | GPU. `@deck.gl/react` + `@deck.gl/layers` + `@deck.gl/aggregation-layers`. |
| Globo 3D de la Tierra, la Luna o Marte, terreno | **CesiumJS** (`cesium` / `resium`) | Sin token de Cesium ion si usas tus propias teselas (Trek, GIBS). |
| Escena 3D libre (nave, hábitat, órbitas) | **Three.js** / `@react-three/fiber` | Para juegos y simulaciones. |
| Cielo (SPHEREx, catálogos) | **Aladin Lite** | Embebible por script, con HiPS. |
| Gráficos estadísticos | **Observable Plot** (rápido y expresivo), **Recharts** (React), **ECharts** (heatmaps y calendarios) | ECharts trae heatmaps de calendario listos. |

Antes de diseñar gráficos, revisa también la skill global `dataviz` si está instalada (paleta, accesibilidad).

## Next.js: los mapas son componentes cliente
```tsx
// src/components/Map.tsx
"use client";
import { useEffect, useRef } from "react";
import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";

const GIBS = (layer: string, date: string) =>
  `https://gibs.earthdata.nasa.gov/wmts/epsg3857/best/${layer}/default/${date}/GoogleMapsCompatible_Level9/{z}/{y}/{x}.jpg`;

export default function Map({ date = "2025-09-15" }: { date?: string }) {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!ref.current) return;
    const map = new maplibregl.Map({
      container: ref.current,
      style: {
        version: 8,
        sources: { gibs: { type: "raster", tiles: [GIBS("MODIS_Terra_CorrectedReflectance_TrueColor", date)], tileSize: 256, maxzoom: 9,
          attribution: "Imagery: NASA GIBS / Worldview" } },
        layers: [{ id: "gibs", type: "raster", source: "gibs" }],
      },
      center: [-58, -23], zoom: 4,
    });
    return () => map.remove();
  }, [date]);
  return <div ref={ref} className="h-[70vh] w-full rounded-lg" role="region" aria-label="Mapa" />;
}
```
En la página, si la librería toca `window` al importar (por ejemplo Cesium), cárgala con `next/dynamic` y `{ ssr: false }` desde un componente cliente.

**CesiumJS en Next.js:** hay que copiar sus assets estáticos (`Workers`, `Assets`, `Widgets`, `ThirdParty`) a `public/cesium/` y definir `window.CESIUM_BASE_URL = "/cesium"` antes de importar. Hazlo en `postinstall` con un script de copia. Para la Luna usa `Cesium.Ellipsoid.MOON` y una capa de imágenes de Trek.

## Rendimiento
- GeoJSON de menos de 5 MB en `public/data/`. Si es más grande: PMTiles (`pmtiles` + protocolo en MapLibre) o agregar en el servidor.
- Simplifica geometrías (`mapshaper -simplify 10%`).
- Series largas: pre-agrega por mes. No mandes datos diarios de 20 años al cliente.
- Lazy-load del mapa debajo del primer pantallazo.

## Accesibilidad y claridad
- Leyenda con unidades y fuente siempre visibles.
- Paletas aptas para daltonismo: viridis o cividis para secuencial, RdBu o BrBG para divergente.
- Cada gráfico con un título que diga la conclusión (ver `data-biz`) y un texto alternativo o tabla de datos.
- Interacción por teclado en los controles (fecha, capa) y un estado vacío o de error claro.
- Atribución de NASA en el mapa ("NASA GIBS", "NASA FIRMS", "NASA/JPL-Caltech/MSSS", etc.).
