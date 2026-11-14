// Bridge to the Bevy WASM sim (public/sim/). Types mirror the Rust JSON in sim/src/bridge.rs.
// Code written with AI assistance (Claude / omp); see docs/SI_USAGE.md.

export type Overlay =
  | "none"
  | "slope"
  | "rough"
  | "texture"
  | "hazard"
  | "thermal"
  | "radiation"
  | "geology"
  | "cost";
export type CameraMode = "map" | "drone" | "rover";

export interface RouteParams {
  body_kg: number;
  suit_kg: number;
  payload_kg: number;
  s_max_base_deg: number;
  w_slope: number;
  w_hazard: number;
  w_thermal: number;
  w_rad: number;
  /** DTM roughness (relief residual) weight. */
  w_rough: number;
  /** HiRISE optical texture (local contrast) weight. */
  w_texture: number;
}

export interface LatLon {
  lat: number;
  lon: number;
}

export interface RouteResult {
  status: "ok" | "no_path";
  length_m: number;
  eta_min: number;
  max_slope_deg: number;
  mean_hazard: number;
  weight_factor: number;
  s_max_eff_deg: number;
  ascent_m: number;
  descent_m: number;
  /** Elevation profile: distance along route (m) and absolute elevation (m), ≤ ~100 points. */
  profile: { d: number; h: number }[];
  start: LatLon;
  end: LatLon;
}

export interface PointInfo extends LatLon {
  elevation_m: number;
  slope_deg: number;
  /** Whether the planner can enter this cell with the current params. */
  passable: boolean;
}

export interface PickEvent extends PointInfo {
  which: "start" | "end";
  /** Sol of the nearest Perseverance waypoint within 400 m, if any (links to its raw images). */
  nearest_sol: number | null;
  nearest_sol_dist_m: number | null;
}

export type PickTarget = "start" | "end";

export interface TilesEvent {
  loading: number;
  detail: number;
}

/** Map camera orientation, for the compass and the 2D/3D button. */
export interface CameraEvent {
  /** Degrees clockwise from north. */
  heading_deg: number;
  /** Degrees from straight down (0 = 2D top view). */
  tilt_deg: number;
}

export interface MarsSimApi {
  set_overlay(name: Overlay): void;
  set_camera_mode(mode: CameraMode): void;
  set_params(json: string): void;
  play_route(play: boolean): void;
  clear_route(): void;
  set_pick_target(which: PickTarget): void;
  /** Vertical exaggeration, 1–5 (default 2). */
  set_vertical_scale(v: number): void;
  set_show_traverse(show: boolean): void;
  /** Back to the map camera, framing the route (or the whole region). */
  reset_view(): void;
  /** Map camera buttons: zoom in (+steps) / out (−steps), north up, 2D (true) / 3D (false). */
  map_zoom(steps: number): void;
  map_north(): void;
  map_top_down(top: boolean): void;
}

declare global {
  interface Window {
    marsSim?: MarsSimApi;
  }
}

// Same defaults as RouteParams::default() in sim/src/route.rs.
// suit_kg 104: NASA Mars suit mass target (NTRS 20260005410). s_max_base_deg: team planning estimate.
export const DEFAULT_PARAMS: RouteParams = {
  body_kg: 80,
  suit_kg: 104,
  payload_kg: 20,
  s_max_base_deg: 20,
  w_slope: 3,
  w_hazard: 5,
  w_thermal: 0.5,
  w_rad: 0.5,
  w_rough: 2,
  w_texture: 1,
};

const TERRAIN_SRC = "HiRISE DTM (NASA/JPL/University of Arizona/USGS) via JPL Mars Trek";

// Hex stops match the colormaps in sim/src/layers.rs.
export const OVERLAY_LEGENDS: Record<Overlay, { label: string; stops: string[]; unit: string; source: string }> = {
  none: { label: "None", stops: [], unit: "", source: "" },
  slope: {
    label: "Slope",
    stops: ["#2e9e5b", "#e8c547", "#d6452f", "#6b2a8c"],
    unit: "0 → allowed max slope; purple = too steep",
    source: TERRAIN_SRC,
  },
  rough: {
    label: "Roughness (DTM)",
    stops: ["#2e9e5b", "#d6452f"],
    unit: "local relief 0 → ≥1 m",
    source: TERRAIN_SRC,
  },
  texture: {
    label: "Surface texture (optical)",
    stops: ["#1d2b53", "#f08a24"],
    unit: "HiRISE local contrast, relative (smooth → speckled/bouldery)",
    source: "HiRISE orthomosaic (NASA/JPL/University of Arizona) via JPL Mars Trek",
  },
  hazard: {
    label: "Hazard (rocks / rough ground)",
    stops: ["#f08a24", "#5a0f1a"],
    unit: "orange = rough (darker = worse), dark red = impassable",
    source: "Team-labelled HiRISE patches + MLP classifier",
  },
  thermal: {
    label: "Thermal swing",
    stops: ["#3b6fd8", "#d6452f"],
    unit: "relative day–night swing (low → high)",
    source: "THEMIS day/night IR mosaics (NASA/JPL/ASU) via JPL Mars Trek",
  },
  radiation: {
    label: "Sky exposure (radiation proxy)",
    stops: ["#f2d43d00", "#f2d43d"],
    unit: "relative sky-view factor; not a dose",
    source: "Derived from HiRISE DTM",
  },
  geology: {
    label: "Geologic units",
    stops: ["#4e79a7", "#f28e2b", "#e15759", "#76b7b2", "#59a14f", "#edc948"],
    unit: "categorical",
    source: "USGS SIM 3464 (Sun & Stack, 2020)",
  },
  cost: {
    label: "Route cost",
    stops: ["#1d2b53", "#f2d43d", "#5a0f1a"],
    unit: "cost ×1 → ×6; dark red = impassable",
    source: "Planner model (slope, roughness, texture, hazard, thermal, exposure)",
  },
};

export const NASA_LIMITS = {
  radiation: {
    career_msv: 600,
    career_cite: "NASA-STD-3001 Vol. 1, 4.8.2 [V1 4030]",
    spe_design_msv: 250,
    spe_cite: "NASA-STD-3001 Vol. 1 [V1 4031]",
  },
  skin_contact: {
    unlimited_min_c: 15,
    unlimited_max_c: 43,
    tissue_damage_cold_c: 0,
    tissue_damage_hot_c: 49,
    cite: "NASA-STD-3001 Vol. 2 [V2 9102/9103]; OCHMO-TB-009",
  },
  suit_pressure_kpa: { emu: 29.6, cite: "OCHMO-TB-050 (EMU operating pressure)" },
  hydration_ml_per_h: { eva_min: 240, cite: "NASA-STD-3001 Vol. 2 [V2 11030]" },
} as const;

export function getSim(): MarsSimApi | null {
  return typeof window === "undefined" ? null : (window.marsSim ?? null);
}

export function onSimEvent<T>(
  name:
    | "mars-sim-ready"
    | "mars-pick"
    | "mars-route"
    | "mars-rover-stopped"
    | "mars-hover"
    | "mars-pick-target"
    | "mars-tiles"
    | "mars-camera",
  cb: (d: T) => void,
): () => void {
  const h = (e: Event) => cb((e as CustomEvent<T>).detail);
  window.addEventListener(name, h);
  return () => window.removeEventListener(name, h);
}
