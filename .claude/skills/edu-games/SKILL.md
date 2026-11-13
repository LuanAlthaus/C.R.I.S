---
name: edu-games
description: Técnicas para construir juegos y experiencias educativas web en un hackathon — arquitectura de simulación por turnos (estado + reglas puras), elección de motor (React + useReducer, Phaser, Kaplay, react-three-fiber), scrollytelling, sonificación con Tone.js / Web Audio, guardado de progreso y accesibilidad para público joven. Úsala cuando el proyecto sea un juego, una simulación interactiva, una historia interactiva o una experiencia sonora.
---

# Juegos y experiencias educativas (técnicas)

> Patrones técnicos, no diseños de juego. Las mecánicas y la idea las define el equipo.

## 1. Objetivos de aprendizaje primero
Antes de programar, el equipo escribe 2 o 3 frases "Al terminar, el jugador entiende que…". Cada mecánica debe servir a una de ellas. Los números del juego salen de **fuentes reales de NASA, citadas en una pantalla "La ciencia detrás"**.

## 2. Arquitectura: estado + acciones + reglas puras
Separar la lógica (pura, testeable) de la UI permite iterar rápido y testear con Vitest.
```ts
// src/game/engine.ts — esqueleto genérico; el equipo define recursos, acciones y reglas
export type State = { turn: number; resources: Record<string, number>; log: string[] };
export type Action = { type: string; payload?: Record<string, number> };
export type Rule = (s: State, a: Action) => State;

export const reduce = (rules: Rule[]) => (s: State, a: Action): State =>
  rules.reduce((acc, r) => r(acc, a), { ...s, turn: s.turn + 1, log: [] });
```
```tsx
// en un componente "use client"
const [state, dispatch] = useReducer(reduce(rules), initialState);
```
- Cada regla agrega a `log` el **porqué** de un cambio. Eso es el feedback educativo.
- Los eventos aleatorios usan una semilla (`seedrandom`) para que la demo sea reproducible.

## 3. Motor
| Tipo | Recomendación |
|---|---|
| Por turnos, tarjetas, paneles | **React + `useReducer`**: lo más rápido |
| Acción 2D, sprites, física simple | **Phaser 3** o **Kaplay**, montado en un componente cliente (`useEffect` crea y destruye el juego) |
| 3D | **@react-three/fiber** + `@react-three/drei` |
| Historia interactiva | Scrollytelling: React + Intersection Observer (o `scrollama`) |

- Phaser y Three.js tocan `window`: cárgalos con `next/dynamic` y `{ ssr: false }` desde un componente cliente.
- Progreso en `localStorage` (con try/catch). Supabase solo para datos compartidos, como un leaderboard con RLS que permita insertar pero no editar.

## 4. Sonificación (Tone.js / Web Audio)
- `pnpm add tone`. El audio necesita un gesto del usuario: `await Tone.start()` en un clic.
- Mapeos clásicos: valor → altura (cuantizada a una escala para que no suene disonante), segunda variable → volumen o filtro, categoría → timbre, tiempo → tempo.
- Sincronización con visuales: `Tone.Transport` + `requestAnimationFrame`. Para leer los píxeles de un frame de video: `<canvas>` + `getImageData` (el video tiene que venir del mismo origen o con CORS).
- Muestra el mapeo en pantalla y ofrece una alternativa sin audio (texto, `aria-live`).

## 5. Público joven y accesibilidad
- Textos cortos, iconos con etiqueta, sin depender solo del color.
- Controles grandes, que funcionen con teclado y en tablet.
- Partidas cortas: el jurado juega poco tiempo.
- **Regla oficial:** el proyecto no puede incluir imágenes, voz ni semejanza de menores de 18 años, ni siquiera en el video.
