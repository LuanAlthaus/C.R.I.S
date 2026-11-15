//! Data overlays draped over the terrain; the texture is recomposed on the CPU when the layer or params change.
//! Hex stops are mirrored in src/lib/marsSim.ts (OVERLAY_LEGENDS).

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::data::{N, Site, VScale};
use crate::terrain::Surface;
use crate::route::{CostGrid, Grid, RouteParams, rough_max, s_max_eff};

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Overlay {
    #[default]
    None,
    Slope,
    Rough,
    Texture,
    Rocks,
    Hazard,
    Thermal,
    Radiation,
    Geology,
    Cost,
}

impl Overlay {
    pub const ALL: [Overlay; 10] = [
        Overlay::None,
        Overlay::Slope,
        Overlay::Rough,
        Overlay::Texture,
        Overlay::Rocks,
        Overlay::Hazard,
        Overlay::Thermal,
        Overlay::Radiation,
        Overlay::Geology,
        Overlay::Cost,
    ];

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "none" => Overlay::None,
            "slope" => Overlay::Slope,
            "rough" => Overlay::Rough,
            "texture" => Overlay::Texture,
            "rocks" => Overlay::Rocks,
            "hazard" => Overlay::Hazard,
            "thermal" => Overlay::Thermal,
            "radiation" => Overlay::Radiation,
            "geology" => Overlay::Geology,
            "cost" => Overlay::Cost,
            _ => return None,
        })
    }
}

const GREEN: [u8; 3] = [0x2e, 0x9e, 0x5b];
const YELLOW: [u8; 3] = [0xe8, 0xc5, 0x47];
const RED: [u8; 3] = [0xd6, 0x45, 0x2f];
const PURPLE: [u8; 3] = [0x6b, 0x2a, 0x8c];
const BLOCKED: [u8; 3] = [0x5a, 0x0f, 0x1a];
const ORANGE: [u8; 3] = [0xf0, 0x8a, 0x24];
const BLUE: [u8; 3] = [0x3b, 0x6f, 0xd8];
const SUN: [u8; 3] = [0xf2, 0xd4, 0x3d];
const NAVY: [u8; 3] = [0x1d, 0x2b, 0x53];
const GEOLOGY: [[u8; 3]; 12] = [
    [0xff, 0x9d, 0xa7],
    [0x4e, 0x79, 0xa7],
    [0xf2, 0x8e, 0x2b],
    [0xe1, 0x57, 0x59],
    [0x76, 0xb7, 0xb2],
    [0x59, 0xa1, 0x4f],
    [0xed, 0xc9, 0x48],
    [0xb0, 0x7a, 0xa1],
    [0x9c, 0x75, 0x5f],
    [0xba, 0xb0, 0xac],
    [0x86, 0xbc, 0xb6],
    [0xd3, 0x72, 0x95],
];
const ALPHA: u8 = 150;

fn lerp(a: [u8; 3], b: [u8; 3], t: f32) -> [u8; 3] {
    let t = t.clamp(0.0, 1.0);
    std::array::from_fn(|k| (a[k] as f32 + (b[k] as f32 - a[k] as f32) * t).round() as u8)
}

fn px(rgb: [u8; 3], a: u8) -> [u8; 4] {
    [rgb[0], rgb[1], rgb[2], a]
}

/// RGBA8 texture (N×N, row 0 = north) for one overlay.
pub fn compose(overlay: Overlay, site: &Site, grid: &CostGrid, p: &RouteParams) -> Vec<u8> {
    let s_eff = s_max_eff(p);
    let (rad_lo, rad_hi) = site.meta.radiation_range().map_or((0.0, 1.0), |r| (r.min, r.max));
    let mut out = Vec::with_capacity(N * N * 4);
    for i in 0..N * N {
        let v = |layer: &[u8]| layer[i] as f32 / 255.0;
        let rgba = match overlay {
            Overlay::None => [0; 4],
            Overlay::Slope => {
                let t = site.slope_deg[i] / s_eff;
                match t {
                    t if t >= 1.0 => px(PURPLE, ALPHA),
                    t if t < 0.5 => px(lerp(GREEN, YELLOW, t * 2.0), ALPHA),
                    t => px(lerp(YELLOW, RED, (t - 0.5) * 2.0), ALPHA),
                }
            }
            // Relief residual against the route's limit: green smooth .. red near the limit, purple = blocked.
            Overlay::Rough => match site.rough_m[i] / rough_max(p).max(1e-3) {
                t if t > 1.0 => px(PURPLE, ALPHA),
                t => px(lerp(GREEN, RED, t), ALPHA),
            },
            Overlay::Texture => px(lerp(NAVY, ORANGE, v(&site.texture)), ALPHA),
            Overlay::Rocks => px(RED, (v(&site.rocks) * 200.0) as u8),
            // The active vehicle's hazard layer (EVA: hazard.png, rover: hazard_phys.png).
            Overlay::Hazard => match grid.hazard_at(i, p) {
                255 => px(BLOCKED, 220),
                h => px(ORANGE, (h as u32 * 200 / 254) as u8),
            },
            Overlay::Thermal => px(lerp(BLUE, RED, v(&site.thermal)), ALPHA),
            // Display stretch over the site's sky-view range so relative differences are visible.
            Overlay::Radiation => {
                let t = ((v(&site.radiation) - rad_lo) / (rad_hi - rad_lo).max(1e-6)).clamp(0.0, 1.0);
                px(SUN, (t * ALPHA as f32) as u8)
            }
            Overlay::Geology => match site.geology[i] {
                0 => [0; 4],
                id => px(GEOLOGY[id as usize % 12], ALPHA),
            },
            Overlay::Cost => {
                match grid.multiplier(i, p) {
                    None => px(BLOCKED, 200),
                    Some(m) => px(lerp(NAVY, SUN, (m.clamp(1.0, 6.0) - 1.0) / 5.0), ALPHA),
                }
            }
        };
        out.extend_from_slice(&rgba);
    }
    out
}

#[derive(Resource)]
pub struct OverlayAssets {
    image: Handle<Image>,
    pub material: Handle<StandardMaterial>,
}

/// Per-chunk overlay child (spawned by terrain, shares the chunk mesh).
#[derive(Component)]
pub struct OverlayMesh;

pub struct LayersPlugin;

impl Plugin for LayersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Overlay>()
            .insert_resource(ShowTraverse(false))
            .add_systems(Startup, setup)
            .add_systems(Update, (recompose, draw_traverse));
    }
}

/// Whether to draw Perseverance's real drive path (waypoints from mars.nasa.gov).
#[derive(Resource)]
pub struct ShowTraverse(pub bool);

fn draw_traverse(show: Res<ShowTraverse>, site: Res<Site>, surface: Res<Surface>, vs: Res<VScale>, mut gizmos: Gizmos) {
    if show.0 && site.traverse.len() > 1 {
        let line = surface.drape(site.traverse.iter().map(|w| Vec2::new(w.x, w.z)), *vs, 0.15, 4.0);
        gizmos.linestrip(line, Color::srgb_u8(0x8f, 0xb8, 0xff));
    }
}

pub fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let image = images.add(Image::new(
        Extent3d { width: N as u32, height: N as u32, depth_or_array_layers: 1 },
        TextureDimension::D2,
        vec![0; N * N * 4],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    ));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image.clone()),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.insert_resource(OverlayAssets { image, material });
}

fn recompose(
    overlay: Res<Overlay>,
    params: Res<RouteParams>,
    site: Res<Site>,
    grid: Res<Grid>,
    assets: Option<Res<OverlayAssets>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut vis: Query<&mut Visibility, With<OverlayMesh>>,
) {
    let Some(assets) = assets else { return };
    let params_matter = matches!(*overlay, Overlay::Slope | Overlay::Rough | Overlay::Hazard | Overlay::Cost);
    if !overlay.is_changed() && !(params.is_changed() && params_matter) {
        return;
    }
    let shown = if *overlay == Overlay::None { Visibility::Hidden } else { Visibility::Visible };
    for mut v in &mut vis {
        *v = shown;
    }
    if *overlay == Overlay::None {
        return;
    }
    if let Some(mut img) = images.get_mut(&assets.image) {
        img.data = Some(compose(*overlay, &site, &grid.0, &params));
    }
    // Touch the material so its bind group picks up the new texture data.
    let _ = materials.get_mut(&assets.material);
}
