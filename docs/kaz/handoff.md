# kaz/handoff — Jezero Marswalk Planner (challenge 10 "Martian Map")

Actualizado: 2026-11-14 15:50 -03. Se actualiza cada hora. Al abrir una sesión nueva: leer este archivo y seguir desde **Siguiente**.

## Cómo retomar
1. Abrir una sesión del agente en `~/Hackaton/nasa-space-apps-starter-template` (Claude Code lee `CLAUDE.md`, que apunta acá).
2. Toolchain: `source ~/.cargo/env` (Rust 1.97.1 + wasm32 + wasm-bindgen-cli 0.2.130), `corepack pnpm`, Python en `.venv` o `uv run --with ...`.
3. Dev server: `corepack pnpm dev` en :3000 (comprobar con `ss -ltnp | grep 3000` antes; no lanzar dos).
4. No correr `pnpm build` y el build de Rust a la vez (OOM). `cargo test` y el build wasm comparten `../mars_sim_target` (lock: uno espera al otro). `wasm-opt` vía npx tarda ~6 min.
5. Metodología del usuario: delegar la recolección de información a subagentes (scout/task) que reportan en español; el agente principal edita.
6. Pruebas en navegador: `uv run -q --with playwright --with pillow --with numpy python /tmp/mw3_check.py`. Usa la GPU Intel (`--use-angle=gl-egl --ignore-gpu-blocklist`, ~32 fps). Con swiftshader el sim va a ~1 fps y los arrastres se leen como clics.

## Pedidos del usuario (sesión 15:00)
1. Rover **idéntico al modelo original**: usar el GLB del equipo `25042_Perseverance.glb` (raíz del repo y `~/Descargas`).
2. Vista rover: texturas mal y el rover/cámara quedaba debajo del terreno al moverse.
3. Líneas azules en el terreno sin inicio/fin elegidos (= recorrido de Perseverance).
4. El dron no se podía girar.
5. Mapa: comportamiento **como Google Maps**. El usuario va a pasar **un mapa real de Marte** para reemplazar el actual: esperar y adaptar el pipeline/terreno.
6. Al terminar: **verificar que las rutas sean las más seguras para el robot y los humanos**.

## Hecho (sesión 15:00–15:50)
- `public/data/jezero/rover.glb` = `25042_Perseverance.glb` del equipo con logos NASA/JPL, bandera e insignia MARS 2020 tapados (texturas `mars_2020_03`, `arm_graphics`, `blade`; el resto byte a byte igual). Frente del modelo +Z, se rota 180° en `rover.rs`. Sin skins ni animaciones. Créditos en README, `SourcesFooter.tsx`, `docs/SI_USAGE.md`.
- `terrain.rs`: recurso `Surface` (alturas de la malla visible: coarse 26² o detail 251², misma triangulación) + `drape()`; `TerrainSet`; mipmaps + anisotropía 16 para ortho global y tiles (`mipmapped`).
- `rover.rs`: carga el GLB (`WorldAssetRoot` = ex `SceneRoot` en Bevy 0.20), pose desde 4 contactos de rueda sobre `Surface` cada frame (`settle`).
- `camera.rs`: Map estilo Google Maps (arrastre con punto agarrado + inercia, rueda hacia el cursor, doble clic acerca, Shift/Ctrl/der. rota+inclina, 2 dedos giran/inclinan, flechas/+/-), evento `mars-camera`; Drone: Q/E y ←/→ giran, ↑/↓ miran, R/F suben/bajan; Rover: chase sobre `Surface`, sube si una loma tapa. Gizmos: `depth_bias` -1 en Map, 0 en Drone/Rover; líneas a 0,15 m del suelo.
- `picking.rs`: `Pointer` con doble clic (los clics simples se reportan tras 0,3 s), twist/arrastre de 2 dedos, `down_at` público; picking sobre `Surface`.
- `layers.rs`: `ShowTraverse(false)` por defecto y línea pegada al terreno. `Planner.tsx` también arranca en false.
- `route.rs`: A* no corta esquinas en diagonal junto a celdas bloqueadas; límite de pendiente estricto (< 20°, regla NASA de EVA); ruta dibujada con `drape`.
- `bridge.rs` + `marsSim.ts`: `map_zoom(steps)`, `map_north()`, `map_top_down(bool)`, evento `mars-camera`. `MapOverlay.tsx`: brújula, +/−, 2D/3D, textos de ayuda nuevos, hover del modo seleccionado arreglado. `MarsSim.tsx`: sin menú contextual en el canvas.
- Verificación: `cargo test --lib` 10/10; `pnpm lint`/`tsc` OK. Navegador (Intel UHD, build anterior al ajuste de líneas): 32 fps, 0 píxeles azules al cargar, ruta ok, arrastre/rueda sin crear puntos, Shift+arrastre gira, brújula a 0°, 2D a 0,5° de inclinación, dron gira, rover GLB apoyado, sin errores de consola.
- Seguridad de rutas (subagente `SafetyLimits` + 12 pares reales): la ruta segura baja la pendiente máx. (p. ej. 17,5° → 4,5°) y las celdas > 15°, con ≤ 4 % más de largo; rugosidad máx. en rutas ≤ 0,24 m.

## Siguiente
1. Hecho a las 16:00: wasm final (26 MB, 15:51) verificado con `/tmp/mw3_check.py` (todo OK, sin errores de consola; en vista rover la línea queda detrás del rover), `pnpm lint` y `pnpm build` OK.
2. Cuando llegue el mapa real de Marte del usuario: integrarlo (pipeline/terreno) manteniendo los controles estilo Google Maps.
3. Con el equipo (decisión de diseño, no de la IA): límite duro de rugosidad (paso con traje 30,5 cm; obstáculo del rover 40 cm), margen bajo 20° (tope lunar de Artemis; no hay cifra para Marte), etiquetar hazard.

## Pendiente del equipo (humano)
- Etiquetar ≥150 parches (≥10 por clase) en `notebooks/labels/jezero_patches.csv` con las hojas de `data/processed/jezero/label_sheets/`; luego `python notebooks/jezero_pipeline.py hazard` y reconstruir el wasm. No etiquetar con IA (regla del evento). Sin esto `hazard.png` está vacío y las rocas no entran en el costo.

## Notas
- No hay `gh` ni token de GitHub en la máquina: las notas van en `docs/kaz/`.
- Sin commits todavía; todo está sin commitear. `25042_Perseverance.glb` en la raíz es del usuario (tiene logos: no commitearlo; la app usa `public/data/jezero/rover.glb`).
