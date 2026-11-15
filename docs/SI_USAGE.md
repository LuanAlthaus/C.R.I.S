# Use of Super Intelligence (SI) — registro del equipo

> Space Apps 2026 llama "Super Intelligence (SI)" a la IA. La entrega **debe** describir cómo se usó. Este archivo es el borrador para el campo "Use of SI" del formulario. Llénalo **durante** el hackathon, no al final.
> Recuerda: la SI **no** puede usarse para idear la solución. Las imágenes y videos generados llevan marca de agua visible, y el código, audio y datos generados se identifican aquí y en el README.

## Template (antes del hackathon)
| Herramienta | Uso | Archivos |
|---|---|---|
| Claude Code (Anthropic) | Investigación de recursos públicos de los challenges, documentación del template, skills y código boilerplate (Next.js + Supabase) | `docs/`, `.claude/skills/` (las propias), `src/` |

## Proyecto (completar desde el sáb 14 nov 9:00)
| Herramienta | Uso | Archivos / assets | ¿Marca de agua o etiqueta? |
|---|---|---|---|
| Claude Code / omp (Claude Opus + subagentes) | Escribió el código a partir del plan del equipo (la idea es del equipo): pipeline de datos en Python, simulación Bevy/Rust (terreno 3D, capas, planificador de rutas A*, rover, cámaras, puente JS), route handlers de Next.js, puente TS y script de build WASM. Investigó endpoints y formatos de las fuentes de datos. | `notebooks/jezero_pipeline.py`, `sim/`, `scripts/build-sim.sh`, `public/sim/boot.js`, `src/lib/marsSim.ts`, `src/components/MarsSim.tsx`, `src/app/api/nasa/donki-mars/`, `src/app/api/nasa/raw-images/` | Etiqueta en texto: comentario de cabecera "Code written with AI assistance" en cada archivo, y crédito en el README y en el footer de la UI |
| omp `frontendagent` (Claude) | Construyó la UI (panel, tarjetas de condiciones, galería de imágenes, fuentes) a partir del contrato de `src/lib/marsSim.ts` | `src/app/page.tsx`, `src/app/layout.tsx`, `src/components/` (excepto `MarsSim.tsx`), `src/app/globals.css` | Crédito en el README y línea en el footer de la UI |
| scikit-learn `MLPClassifier` (modelo propio, no generativo) | Clasifica parches HiRISE de 48 m en seguro / rugoso / intransitable para la capa de peligro. Se entrena con las etiquetas del equipo si hay 150 o más; si no, con las etiquetas por reglas de la etapa `autolabel` | `notebooks/labels/jezero_patches.csv` (etiquetas humanas), `notebooks/labels/jezero_patches_nasa.csv` (etiquetas por reglas), `sim/assets/jezero/hazard.png` (salida del modelo) | `meta.json` → `hazard_model` (`labels`: `team` o `nasa_rules`, n.º de etiquetas, exactitud con validación espacial) y crédito de la capa en la UI |

Datos hechos por humanos: las etiquetas de `notebooks/labels/jezero_patches.csv` las pone el equipo a mano (no SI).

Datos generados con SI: `notebooks/labels/jezero_patches_nasa.csv`. Claude/omp escribió las **reglas** (etapa `autolabel`) a pedido del equipo; ninguna persona ni IA mira los parches uno por uno. Cada etiqueta sale de datos de NASA y de criterios numéricos publicados por NASA, y la columna `rule` dice qué regla la puso.
- Pendiente p90 ≥ 20°: límite de EVA de NASA.
- Mapas geológicos del equipo de Mars 2020: Stack et al. 2020 y Crumpler et al. 2023.
- Recorrido real de Perseverance.

v2 (14 nov, tarde): Claude/omp reescribió el pipeline para 10 × 10 km con tiles de detalle (`public/data/jezero/tiles/`), capas derivadas de rugosidad (DTM) y textura óptica (HiRISE), etapa `traverse` (waypoints de Perseverance desde mars.nasa.gov → `traverse.json`), streaming LOD de terreno, cámaras nuevas, modelo de rover estilo Curiosity hecho con primitivas (código, no imagen generada) y la UI v2 (`frontendagent`). Notas de traspaso: `docs/kaz/handoff.md`.

v2.1 (14 nov, 15:00–16:00): Claude/omp (agente principal + subagentes de investigación) cambió el rover de primitivas por el modelo 3D de Perseverance que aportó el equipo (`25042_Perseverance.glb`, NASA/JPL-Caltech, NASA 3D Resources). Un subagente pintó encima de los logos de NASA/JPL, la bandera y la insignia de Mars 2020 en 3 texturas (`mars_2020_03`, `arm_graphics`, `blade`) con el color de alrededor: es una edición del asset, no una imagen generada → `public/data/jezero/rover.glb`. También escribió código: alturas de la malla visible (`Surface`) para rover, cámaras y clics; mipmaps de las texturas del terreno; cámara de mapa al estilo Google Maps; giro del dron por teclado; recorrido de Perseverance oculto por defecto; A* sin cortar esquinas en diagonal; y botones de brújula, zoom y 2D/3D en la UI. Archivos: `sim/src/{terrain,rover,camera,picking,route,layers,bridge,data}.rs`, `src/lib/marsSim.ts`, `src/components/MarsSim.tsx`, `src/components/Planner.tsx`, `src/components/planner/{MapOverlay,SourcesFooter}.tsx`. Los subagentes investigaron el modelo de NASA, la API glTF de Bevy, los controles de Google Maps y los límites de seguridad del rover y de la EVA.

v2.2 (14 nov, 16:00–17:00): a pedido del equipo, Claude/omp escribió código para: límite duro de rugosidad (`rough_max_m`, 0,30 m por defecto, editable en la UI); capa y peso de rocas (`w_rocks`) a partir del mapa fotogeológico oficial del Mars 2020 Science Team (Stack et al. 2020: talud y roca madre "blocky" expuesta); color real del terreno a partir del mapa base de color de Mars 2020 (HiRISE RGB + CTX), que tiñe el ortho gris de HiRISE; y relieve real (exageración 1×) por defecto. Archivos: `notebooks/jezero_pipeline.py` (etapas `color` y `rocks`), `sim/src/{route,layers,data,terrain,input}.rs`, `src/lib/marsSim.ts`, `src/components/Planner.tsx`, `src/components/planner/{LimitsSection,LayersSection}.tsx`. Los subagentes buscaron productos oficiales de rocas, capas de color y DEM, y APIs de NASA vigentes. Qué unidades cuentan como "roca" y el peso por defecto (3) son elecciones editables del equipo.

v2.3 (14 nov, 17:00, rama `kaz-tests`): a pedido del equipo ("¿la clasificación podrías hacerla tú en base a los datos de la NASA?"), Claude/omp agregó la etapa `autolabel` (etiquetas por reglas a partir de datos de NASA, descrita arriba) y cambió `hazard`:
- elige solas las etiquetas del equipo cuando hay 150 o más;
- balancea las clases;
- valida con 20 tiles de 1 km apartados.

Resultado: 21 122 de 43 264 parches etiquetados y 93,1 % de exactitud en la validación espacial. Un subagente buscó las fuentes de etiquetas (SPOC, AI4Mars, HiRISE labeled, Crumpler, recorrido de M2020, criterios de JPL y NASA HRP). Los umbrales (mitad del parche, p90) y qué unidades cuentan como rugosas son elecciones editables del equipo.

v2.4 (15 nov, rama `kaz-texture`): a pedido del equipo (texturas de baja calidad en Rover/Drone, verificar que las rutas sean las seguras, icono de info sobre el destino), Claude/omp (agente principal + subagentes de lectura) escribió código:
- `sim/src/terrain.rs`: arreglo de la carga de las teselas de detalle (Bevy 0.20 expandía las alturas PNG de 16 bits a RGBA y ninguna tesela de 1 m se aplicaba); mipmaps en espacio lineal; menos brillo ambiente y reflectancia; un **normal map procedural de grano fino generado por código** (ruido de valor periódico, no es una imagen generada por IA); y un nivel de ortofoto z17 cerca de la cámara.
- `notebooks/jezero_pipeline.py`: etapa `ortho2`, que baja el mismo ortomosaico HiRISE de JPL Mars Trek a zoom 17 (~0,32 m/px) y lo colorea igual que `color` → `public/data/jezero/tiles/o2_*.jpg`. Son datos de NASA reprocesados, no generados.
- `sim/src/route.rs`: métricas nuevas de la ruta, ruta base (la permitida más corta, mismos límites duros y sin pesos), conteo de celdas prohibidas en la línea recta, y tests de seguridad con datos reales.
- `sim/src/picking.rs` + `src/components/planner/RouteInfoPin.tsx`: botón de info sobre el pin de destino que explica por qué la ruta es la más segura.

v2.5 (15 nov, rama `kaz-textures-3D-renew`): a pedido del equipo (modelado 3D más realista a partir de las texturas y comprobar las rutas sobre el terreno nuevo), Claude/omp escribió código. La lectura del repo y la búsqueda web las hicieron tres subagentes con Claude Haiku 4.5; sus cifras de geometría solar y de dunas no se usaron sin comprobar (la dirección del sol se ajustó contra el DTM).
- `sim/src/relief.rs` (nuevo): relieve fino estimado por fotoclinometría a partir de la ortofoto HiRISE z17. Es un **dato derivado por código, no generado por IA**, y es una estimación visual: acuerda poco con el DTM (r ≈ 0,26) y no entra en el cálculo de rutas.
- `sim/src/terrain.rs`: los chunks a menos de 150 m usan una malla de 2 m con ese relieve y `Surface` devuelve esas alturas.
- `sim/src/rover.rs`, `sim/src/route.rs`: test con datos reales que conduce el rover por las rutas sobre el terreno nuevo; la línea de la ruta se muestrea cada 1 m.
- `src/components/planner/SourcesFooter.tsx`: la lista de fuentes declara el relieve estimado.
- `sim/src/rover.rs`, `sim/src/route.rs`, `sim/src/picking.rs`, `sim/src/bridge.rs`, `src/components/Planner.tsx`: arreglo del final del recorrido (el rover para sobre el destino, se queda ahí y el siguiente clic planifica desde esa posición).
