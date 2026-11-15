//! Streamed terrain: 10×10 chunks of 1 km. Every chunk starts as a coarse mesh (40 m spacing, region-wide
//! ortho); chunks near the camera load a detail tile (4 m heights + ~1 m HiRISE ortho) from
//! public/data/jezero/tiles and drop it again when the camera moves away; chunks within 400 m swap to a
//! 2048 px ortho (Trek z17, ~0.5 m/px).

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use bevy::light::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver};
use bevy::math::Affine2;
use bevy::mesh::{Indices, PrimitiveTopology, UvChannel};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::data::{HALF_M, SIZE_M, Site, VScale};
use crate::layers::{OverlayAssets, OverlayMesh};

pub const TILE_M: f32 = 1000.0;
pub const NT: usize = (SIZE_M / TILE_M) as usize;
const COARSE_VERTS: usize = 26; // 40 m spacing
const DETAIL_VERTS: usize = 251; // 4 m spacing
const SKIRT_M: f32 = 15.0;
/// Load a detail tile when the camera is closer than this (m), drop it beyond `DROP_DIST`.
const LOAD_DIST: f32 = 1600.0;
const DROP_DIST: f32 = 2600.0;
const MAX_LOADING: usize = 4;
const MAX_DETAIL: usize = 20;
/// Near-camera z17 ortho: load below `SHARP_LOAD_DIST` (m), drop beyond `SHARP_DROP_DIST`. 2048² RGBA
/// with mips is ~22 MB of VRAM per chunk, hence the cap.
const SHARP_LOAD_DIST: f32 = 400.0;
const SHARP_DROP_DIST: f32 = 700.0;
const MAX_SHARP: usize = 4;
/// Fresnel reflectance of the regolith. Bevy's 0.5 default washes the ground out at the grazing
/// angles of the rover and drone views.
const REFLECTANCE: f32 = 0.15;
/// Fine grain normal map on detail chunks: repeats every `GRAIN_PERIOD_M`, ~1.6 cm per texel.
const GRAIN_PERIOD_M: f32 = 8.0;
const GRAIN_PX: usize = 512;
const GRAIN_STRENGTH: f32 = 0.6;

#[derive(Component)]
pub struct Terrain;

enum Lod {
    Coarse,
    Loading { h: Handle<Image>, o: Handle<Image> },
    /// `material` is the 1 m ortho material, kept to fall back to when the sharp level is dropped.
    Detail { material: Handle<StandardMaterial>, sharp: Sharp },
    Failed,
}

enum Sharp {
    Off,
    Loading(Handle<Image>),
    On,
    /// No z17 tile (outside its footprint) or it failed; not retried until the chunk goes coarse.
    Missing,
}

#[derive(Component)]
pub struct Chunk {
    tx: usize,
    ty: usize,
    lod: Lod,
}

#[derive(Resource)]
struct CoarseAssets {
    material: Handle<StandardMaterial>,
    meshes: Vec<Handle<Mesh>>,
    grain: Handle<Image>,
}

/// Heights (relative m, unexaggerated) of the terrain meshes actually on screen: per chunk, the coarse
/// grid or the detail tile once loaded. Cameras, the rover, pins and lines sample this instead of the
/// 10 m planning grid, so nothing sinks into or floats above the visible ground.
#[derive(Resource)]
pub struct Surface {
    coarse: Vec<Vec<f32>>,
    detail: Vec<Option<Vec<f32>>>,
}

impl Surface {
    /// Height of the rendered mesh at world x/z, interpolated on the same triangles the mesh uses.
    pub fn height(&self, x: f32, z: f32) -> f32 {
        let tile = |v: f32| ((v + HALF_M) / TILE_M).clamp(0.0, NT as f32 - 1e-3) as usize;
        let (tx, ty) = (tile(x), tile(z));
        let i = ty * NT + tx;
        let (h, n) = match &self.detail[i] {
            Some(d) => (d.as_slice(), DETAIL_VERTS),
            None => (self.coarse[i].as_slice(), COARSE_VERTS),
        };
        let (x0, z0) = chunk_origin(tx, ty);
        let s = TILE_M / (n - 1) as f32;
        let u = ((x - x0) / s).clamp(0.0, (n - 1) as f32 - 1e-3);
        let v = ((z - z0) / s).clamp(0.0, (n - 1) as f32 - 1e-3);
        let (i0, j0) = (u as usize, v as usize);
        let (fu, fv) = (u - i0 as f32, v - j0 as f32);
        let at = |i: usize, j: usize| h[j * n + i];
        let (h00, h10, h01, h11) = (at(i0, j0), at(i0 + 1, j0), at(i0, j0 + 1), at(i0 + 1, j0 + 1));
        // grid_mesh splits each quad along the v10-v01 diagonal.
        if fu + fv <= 1.0 {
            h00 + fu * (h10 - h00) + fv * (h01 - h00)
        } else {
            h11 + (1.0 - fu) * (h01 - h11) + (1.0 - fv) * (h10 - h11)
        }
    }

    /// Rendered ground height with vertical exaggeration.
    pub fn ground(&self, x: f32, z: f32, vs: VScale) -> f32 {
        self.height(x, z) * vs.0
    }

    /// Line through the x/z points draped on the rendered ground, `lift_m` above it, sampled every
    /// `step_m` so it doesn't cut through bumps between the points.
    pub fn drape(&self, pts: impl IntoIterator<Item = Vec2>, vs: VScale, lift_m: f32, step_m: f32) -> Vec<Vec3> {
        let mut out = Vec::new();
        let mut prev: Option<Vec2> = None;
        let mut push = |q: Vec2| out.push(Vec3::new(q.x, self.ground(q.x, q.y, vs) + lift_m, q.y));
        for p in pts {
            match prev {
                Some(from) => {
                    let n = (from.distance(p) / step_m).ceil().max(1.0) as usize;
                    (1..=n).for_each(|k| push(from.lerp(p, k as f32 / n as f32)));
                }
                None => push(p),
            }
            prev = Some(p);
        }
        out
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.80, 0.66, 0.52)))
            .insert_resource(GlobalAmbientLight { color: Color::srgb(1.0, 0.86, 0.74), brightness: 180.0, ..default() })
            .init_resource::<TileStats>()
            .add_systems(Startup, setup.after(crate::layers::setup))
            .add_systems(Update, (stream_chunks, finish_loads, finish_sharp, apply_vscale).chain().in_set(TerrainSet));
    }
}

/// Terrain streaming; systems that sample `Surface` run after it.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerrainSet;

/// Grid mesh with vertices at world x/z, heights from `h(i, j)` (i = east index, j = south index),
/// UVs in region space (0..1), UV1 in grain periods, tangents for the grain normal map, and a skirt
/// hanging below the edges to hide LOD cracks.
fn grid_mesh(n: usize, x0: f32, z0: f32, spacing: f32, h: impl Fn(usize, usize) -> f32) -> Mesh {
    let mut pos = Vec::with_capacity(n * n + 4 * n);
    let mut nor = Vec::with_capacity(n * n + 4 * n);
    let mut uv = Vec::with_capacity(n * n + 4 * n);
    let mut uv1 = Vec::with_capacity(n * n + 4 * n);
    let mut tan = Vec::with_capacity(n * n + 4 * n);
    let hc = |i: isize, j: isize| h(i.clamp(0, n as isize - 1) as usize, j.clamp(0, n as isize - 1) as usize);
    for j in 0..n {
        for i in 0..n {
            let (x, z) = (x0 + i as f32 * spacing, z0 + j as f32 * spacing);
            let (ii, jj) = (i as isize, j as isize);
            let dx = (hc(ii + 1, jj) - hc(ii - 1, jj)) / 2.0;
            let dz = (hc(ii, jj + 1) - hc(ii, jj - 1)) / 2.0;
            pos.push([x, h(i, j), z]);
            nor.push(Vec3::new(-dx, spacing, -dz).normalize().to_array());
            let (u0, v0) = ((x + HALF_M) / SIZE_M, (z + HALF_M) / SIZE_M);
            uv.push([u0, v0]);
            // Region UV scaled so the detail material's uv_transform (×NT, integer shift) gives one
            // grain period per GRAIN_PERIOD_M.
            uv1.push([u0 * TILE_M / GRAIN_PERIOD_M, v0 * TILE_M / GRAIN_PERIOD_M]);
            let t = Vec3::new(spacing, dx, 0.0).normalize(); // along +x (+U), orthogonal to the normal
            tan.push([t.x, t.y, t.z, 1.0]);
        }
    }
    let mut idx: Vec<u32> = Vec::with_capacity((n - 1) * (n - 1) * 6 + 4 * (n - 1) * 12);
    for j in 0..n - 1 {
        for i in 0..n - 1 {
            let v00 = (j * n + i) as u32;
            let (v10, v01, v11) = (v00 + 1, v00 + n as u32, v00 + n as u32 + 1);
            idx.extend_from_slice(&[v00, v01, v10, v10, v01, v11]);
        }
    }
    // Skirts: duplicate each edge vertex SKIRT_M lower; both windings so they show from either side.
    let edges: [Vec<usize>; 4] = [
        (0..n).collect(),
        (0..n).map(|i| (n - 1) * n + i).collect(),
        (0..n).map(|j| j * n).collect(),
        (0..n).map(|j| j * n + n - 1).collect(),
    ];
    for edge in edges {
        let base = pos.len() as u32;
        for &v in &edge {
            let [x, y, z] = pos[v];
            pos.push([x, y - SKIRT_M, z]);
            nor.push(nor[v]);
            uv.push(uv[v]);
            uv1.push(uv1[v]);
            tan.push(tan[v]);
        }
        for k in 0..edge.len() - 1 {
            let (a, b) = (edge[k] as u32, edge[k + 1] as u32);
            let (c, d) = (base + k as u32, base + k as u32 + 1);
            idx.extend_from_slice(&[a, c, b, b, c, d, a, b, c, b, d, c]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nor)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_1, uv1)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tan)
        .with_inserted_indices(Indices::U32(idx))
}

/// RGBA8 texture with a box-filtered mip chain and anisotropic sampling. Without mips the ortho
/// shimmers and smears at the grazing angles of the rover and drone views. `srgb` colour is averaged
/// in linear light (averaging the gamma bytes darkens and muddies every level); alpha and non-colour
/// data (normal maps) are averaged as stored.
fn mipmapped(w: u32, h: u32, mut data: Vec<u8>, srgb: bool, address: ImageAddressMode) -> Image {
    let to_lin: [f32; 256] = std::array::from_fn(|i| {
        let c = i as f32 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    });
    let to_srgb = |l: f32| {
        let c = if l <= 0.003_130_8 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 };
        (c * 255.0).round().clamp(0.0, 255.0) as u8
    };
    let (mut lw, mut lh, mut off, mut levels) = (w as usize, h as usize, 0, 1);
    while lw > 1 || lh > 1 {
        let (nw, nh) = ((lw / 2).max(1), (lh / 2).max(1));
        let mut next = Vec::with_capacity(nw * nh * 4);
        for y in 0..nh {
            for x in 0..nw {
                let px = |dx: usize, dy: usize| off + ((2 * y + dy).min(lh - 1) * lw + (2 * x + dx).min(lw - 1)) * 4;
                let (a, b, c, d) = (px(0, 0), px(1, 0), px(0, 1), px(1, 1));
                for k in 0..4 {
                    next.push(if srgb && k < 3 {
                        let lin = |i: usize| to_lin[data[i + k] as usize];
                        to_srgb((lin(a) + lin(b) + lin(c) + lin(d)) / 4.0)
                    } else {
                        let sum = data[a + k] as u32 + data[b + k] as u32 + data[c + k] as u32 + data[d + k] as u32;
                        ((sum + 2) / 4) as u8
                    });
                }
            }
        }
        off += lw * lh * 4;
        data.extend_from_slice(&next);
        (lw, lh, levels) = (nw, nh, levels + 1);
    }
    let size = Extent3d { width: w, height: h, depth_or_array_layers: 1 };
    let format = if srgb { TextureFormat::Rgba8UnormSrgb } else { TextureFormat::Rgba8Unorm };
    let mut img = Image::new_uninit(size, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    img.data = Some(data);
    img.texture_descriptor.mip_level_count = levels;
    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        anisotropy_clamp: 16,
        address_mode_u: address,
        address_mode_v: address,
        ..ImageSamplerDescriptor::linear()
    });
    img
}

/// Tileable tangent-space normal map of fine regolith grain: 4 octaves of periodic value noise.
/// Adds relief below the 4 m mesh spacing and the ~1 m ortho without a custom shader.
fn grain_normal_map() -> Image {
    let n = GRAIN_PX;
    let hash = |x: u32, y: u32, o: u32| {
        let mut h = x.wrapping_mul(0x8da6_b343) ^ y.wrapping_mul(0xd816_3841) ^ o.wrapping_mul(0xcb1a_b31f);
        h ^= h >> 13;
        h = h.wrapping_mul(0x5bd1_e995);
        h ^= h >> 15;
        h as f32 / u32::MAX as f32
    };
    let smooth = |t: f32| t * t * (3.0 - 2.0 * t);
    let mut height = vec![0f32; n * n];
    for (o, (g, amp)) in [(8usize, 1.0f32), (16, 0.5), (32, 0.25), (64, 0.125)].into_iter().enumerate() {
        let cell = n / g;
        let v = |i: usize, j: usize| hash((i % g) as u32, (j % g) as u32, o as u32);
        for y in 0..n {
            for x in 0..n {
                let (i, j) = (x / cell, y / cell);
                let fx = smooth((x % cell) as f32 / cell as f32);
                let fy = smooth((y % cell) as f32 / cell as f32);
                let top = v(i, j) + (v(i + 1, j) - v(i, j)) * fx;
                let bot = v(i, j + 1) + (v(i + 1, j + 1) - v(i, j + 1)) * fx;
                height[y * n + x] += amp * (top + (bot - top) * fy);
            }
        }
    }
    // Slopes per finest-octave cell (8 px), so GRAIN_STRENGTH reads as a tilt factor, not per-texel noise.
    let k = GRAIN_STRENGTH * (n / 64) as f32 / 2.0;
    let mut data = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let at = |x: usize, y: usize| height[(y % n) * n + x % n];
            let dhx = at(x + 1, y) - at(x + n - 1, y);
            let dhy = at(x, y + 1) - at(x, y + n - 1);
            let nv = Vec3::new(-dhx * k, -dhy * k, 1.0).normalize();
            data.extend(nv.to_array().map(|c| ((c * 0.5 + 0.5) * 255.0).round() as u8));
            data.push(255);
        }
    }
    mipmapped(n as u32, n as u32, data, false, ImageAddressMode::Repeat)
}

/// Material of a detail chunk: its own ortho tile plus the grain normal map on UV1.
fn detail_material(texture: Handle<Image>, tx: usize, ty: usize, grain: &Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture),
        // Region UV -> tile UV for this chunk's own ortho texture (also applied to UV1).
        uv_transform: Affine2::from_scale_angle_translation(Vec2::splat(NT as f32), 0.0, -Vec2::new(tx as f32, ty as f32)),
        normal_map_texture: Some(grain.clone()),
        normal_map_channel: UvChannel::Uv1,
        perceptual_roughness: 0.95,
        reflectance: REFLECTANCE,
        ..default()
    }
}

fn chunk_origin(tx: usize, ty: usize) -> (f32, f32) {
    (tx as f32 * TILE_M - HALF_M, ty as f32 * TILE_M - HALF_M)
}

pub fn setup(
    mut commands: Commands,
    site: Res<Site>,
    vs: Res<VScale>,
    overlay: Res<OverlayAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let (w, h) = site.ortho.dimensions();
    let rgba: Vec<u8> = site.ortho.pixels().flat_map(|p| [p[0], p[1], p[2], 255]).collect();
    let ortho = images.add(mipmapped(w, h, rgba, true, ImageAddressMode::ClampToEdge));
    let grain = images.add(grain_normal_map());
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(ortho),
        perceptual_roughness: 0.95,
        reflectance: REFLECTANCE,
        ..default()
    });
    let spacing = TILE_M / (COARSE_VERTS - 1) as f32;
    let mut coarse = Vec::with_capacity(NT * NT);
    let mut coarse_h = Vec::with_capacity(NT * NT);
    for ty in 0..NT {
        for tx in 0..NT {
            let (x0, z0) = chunk_origin(tx, ty);
            let h: Vec<f32> = (0..COARSE_VERTS * COARSE_VERTS)
                .map(|k| {
                    let (i, j) = (k % COARSE_VERTS, k / COARSE_VERTS);
                    site.height_at(x0 + i as f32 * spacing, z0 + j as f32 * spacing)
                })
                .collect();
            let mesh = meshes.add(grid_mesh(COARSE_VERTS, x0, z0, spacing, |i, j| h[j * COARSE_VERTS + i]));
            coarse_h.push(h);
            coarse.push(mesh.clone());
            commands
                .spawn((
                    Terrain,
                    Chunk { tx, ty, lod: Lod::Coarse },
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_scale(Vec3::new(1.0, vs.0, 1.0)),
                ))
                .with_child((
                    OverlayMesh,
                    Mesh3d(mesh),
                    MeshMaterial3d(overlay.material.clone()),
                    Transform::from_xyz(0.0, 0.4, 0.0),
                    Visibility::Hidden,
                    NotShadowCaster,
                    NotShadowReceiver,
                ));
        }
    }
    commands.insert_resource(CoarseAssets { material, meshes: coarse, grain });
    commands.insert_resource(Surface { coarse: coarse_h, detail: vec![None; NT * NT] });

    // Sun 35° above the west-north-west horizon (x = east, z = south).
    let az = 292.5f32.to_radians();
    let el = 35f32.to_radians();
    let to_sun = Vec3::new(el.cos() * az.sin(), el.sin(), -el.cos() * az.cos());
    commands.spawn((
        DirectionalLight { illuminance: 12_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_translation(to_sun).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder { maximum_distance: 3000.0, first_cascade_far_bound: 200.0, ..default() }.build(),
    ));
}

/// Number of chunks loading / at full detail / with the z17 ortho, for the UI.
#[derive(Resource, Default, PartialEq)]
pub struct TileStats {
    pub loading: usize,
    pub detail: usize,
    pub sharp: usize,
}

fn stream_chunks(
    cams: Query<&GlobalTransform, With<Camera3d>>,
    site: Res<Site>,
    vs: Res<VScale>,
    assets: Res<AssetServer>,
    coarse: Option<Res<CoarseAssets>>,
    mut chunks: Query<(&mut Chunk, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>, &Children)>,
    mut overlays: Query<&mut Mesh3d, (With<OverlayMesh>, Without<Chunk>)>,
    mut surface: ResMut<Surface>,
) {
    let (Ok(cam), Some(coarse)) = (cams.single(), coarse) else { return };
    let eye = cam.translation();
    let dist = |tx: usize, ty: usize| {
        let (x0, z0) = chunk_origin(tx, ty);
        let dx = (x0 - eye.x).max(eye.x - x0 - TILE_M).max(0.0);
        let dz = (z0 - eye.z).max(eye.z - z0 - TILE_M).max(0.0);
        let cx = (x0 + TILE_M / 2.0, z0 + TILE_M / 2.0);
        let dy = (eye.y - site.ground(cx.0, cx.1, *vs)).max(0.0);
        (dx * dx + dz * dz + dy * dy).sqrt()
    };

    let main_only = |s: &mut ImageLoaderSettings| s.asset_usage = RenderAssetUsages::MAIN_WORLD;
    let mut loading = 0;
    let mut detail = 0;
    let mut wanted: Vec<(f32, usize, usize)> = Vec::new();
    for (mut chunk, mut mesh, mut mat, children) in &mut chunks {
        let d = dist(chunk.tx, chunk.ty);
        match chunk.lod {
            Lod::Detail { .. } | Lod::Loading { .. } if d > DROP_DIST => {
                let i = chunk.ty * NT + chunk.tx;
                mesh.0 = coarse.meshes[i].clone();
                mat.0 = coarse.material.clone();
                for c in children.iter() {
                    if let Ok(mut m) = overlays.get_mut(c) {
                        m.0 = coarse.meshes[i].clone();
                    }
                }
                surface.detail[i] = None;
                chunk.lod = Lod::Coarse;
            }
            Lod::Coarse if d < LOAD_DIST => wanted.push((d, chunk.tx, chunk.ty)),
            Lod::Loading { .. } => loading += 1,
            Lod::Detail { .. } => detail += 1,
            _ => {}
        }
    }
    wanted.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, tx, ty) in wanted {
        if loading >= MAX_LOADING || loading + detail >= MAX_DETAIL {
            break;
        }
        let Some((mut chunk, ..)) = chunks.iter_mut().find(|(c, ..)| c.tx == tx && c.ty == ty) else { continue };
        // 16-bit grey heights must stay R16: Bevy expands grey PNGs to RGBA16 by default.
        let heights = |s: &mut ImageLoaderSettings| {
            s.asset_usage = RenderAssetUsages::MAIN_WORLD;
            s.expand_grayscale = false;
        };
        let h = assets.load_builder().with_settings(heights).load(format!("tiles/h_{ty}_{tx}.png"));
        let o = assets.load_builder().with_settings(main_only).load(format!("tiles/o_{ty}_{tx}.jpg"));
        chunk.lod = Lod::Loading { h, o };
        loading += 1;
    }

    // Second level on detail chunks: the z17 ortho near the camera, back to the 1 m one further away.
    let mut busy = 0;
    let mut near: Vec<(f32, usize, usize)> = Vec::new();
    for (mut chunk, _, mut mat, _) in &mut chunks {
        let (tx, ty) = (chunk.tx, chunk.ty);
        let d = dist(tx, ty);
        let Lod::Detail { material, sharp } = &mut chunk.lod else { continue };
        match sharp {
            Sharp::On | Sharp::Loading(_) if d > SHARP_DROP_DIST => {
                mat.0 = material.clone();
                *sharp = Sharp::Off; // dropping a Loading handle cancels the load
            }
            Sharp::On | Sharp::Loading(_) => busy += 1,
            Sharp::Off if d < SHARP_LOAD_DIST => near.push((d, tx, ty)),
            _ => {}
        }
    }
    near.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, tx, ty) in near {
        if busy >= MAX_SHARP {
            break;
        }
        let Some((mut chunk, ..)) = chunks.iter_mut().find(|(c, ..)| c.tx == tx && c.ty == ty) else { continue };
        if let Lod::Detail { sharp, .. } = &mut chunk.lod {
            *sharp = Sharp::Loading(assets.load_builder().with_settings(main_only).load(format!("tiles/o2_{ty}_{tx}.jpg")));
            busy += 1;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn finish_loads(
    site: Res<Site>,
    assets: Res<AssetServer>,
    coarse: Option<Res<CoarseAssets>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: Query<(&mut Chunk, &mut Mesh3d, &mut MeshMaterial3d<StandardMaterial>, &Children)>,
    mut overlays: Query<&mut Mesh3d, (With<OverlayMesh>, Without<Chunk>)>,
    mut stats: ResMut<TileStats>,
    mut surface: ResMut<Surface>,
) {
    let Some(coarse) = coarse else { return };
    let (mut loading, mut detail, mut sharp_on) = (0, 0, 0);
    for (mut chunk, mut mesh, mut mat, children) in &mut chunks {
        let Lod::Loading { h, o } = &chunk.lod else {
            if let Lod::Detail { sharp, .. } = &chunk.lod {
                detail += 1;
                sharp_on += matches!(sharp, Sharp::On) as usize;
            }
            continue;
        };
        let failed = |id: AssetId<Image>| matches!(assets.load_state(id), bevy::asset::LoadState::Failed(_));
        if failed(h.id()) || failed(o.id()) {
            warn!("detail tile {}_{} failed to load", chunk.ty, chunk.tx);
            chunk.lod = Lod::Failed;
            continue;
        }
        let (Some(himg), Some(oimg)) = (images.get(h), images.get(o)) else {
            loading += 1;
            continue;
        };
        let Some(bytes) = himg.data.as_ref() else { continue };
        let Ok(ortho) = oimg.clone().try_into_dynamic().map(|d| d.into_rgba8()) else {
            warn!("detail tile {}_{} ortho has an unsupported format", chunk.ty, chunk.tx);
            chunk.lod = Lod::Failed;
            continue;
        };
        let span = site.meta.h_max - site.meta.h_min;
        let mid = site.h_mid();
        let q: Vec<f32> = bytes
            .chunks_exact(2)
            .map(|b| site.meta.h_min + u16::from_le_bytes([b[0], b[1]]) as f32 / 65535.0 * span - mid)
            .collect();
        if q.len() != DETAIL_VERTS * DETAIL_VERTS {
            warn!("detail tile {}_{} has unexpected size", chunk.ty, chunk.tx);
            chunk.lod = Lod::Failed;
            continue;
        }
        let (x0, z0) = chunk_origin(chunk.tx, chunk.ty);
        let spacing = TILE_M / (DETAIL_VERTS - 1) as f32;
        let handle = meshes.add(grid_mesh(DETAIL_VERTS, x0, z0, spacing, |i, j| q[j * DETAIL_VERTS + i]));
        let (ow, oh) = ortho.dimensions();
        let texture = images.add(mipmapped(ow, oh, ortho.into_raw(), true, ImageAddressMode::ClampToEdge));
        mesh.0 = handle.clone();
        for c in children.iter() {
            if let Ok(mut m) = overlays.get_mut(c) {
                m.0 = handle.clone();
            }
        }
        let material = materials.add(detail_material(texture, chunk.tx, chunk.ty, &coarse.grain));
        mat.0 = material.clone();
        surface.detail[chunk.ty * NT + chunk.tx] = Some(q);
        chunk.lod = Lod::Detail { material, sharp: Sharp::Off };
        detail += 1;
    }
    let new = TileStats { loading, detail, sharp: sharp_on };
    if *stats != new {
        crate::bridge::emit(
            "mars-tiles",
            format!(r#"{{"loading":{loading},"detail":{detail},"sharp":{sharp_on}}}"#),
        );
        *stats = new;
    }
}

/// Swap in the z17 ortho of chunks whose `Sharp::Loading` image arrived.
fn finish_sharp(
    assets: Res<AssetServer>,
    coarse: Option<Res<CoarseAssets>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks: Query<(&mut Chunk, &mut MeshMaterial3d<StandardMaterial>)>,
) {
    let Some(coarse) = coarse else { return };
    for (mut chunk, mut mat) in &mut chunks {
        let (tx, ty) = (chunk.tx, chunk.ty);
        let Lod::Detail { sharp, .. } = &mut chunk.lod else { continue };
        let Sharp::Loading(h) = sharp else { continue };
        if matches!(assets.load_state(h.id()), bevy::asset::LoadState::Failed(_)) {
            // Expected for chunks outside the z17 footprint: the pipeline skips them on purpose.
            debug!("no z17 ortho for chunk {ty}_{tx}");
            *sharp = Sharp::Missing;
            continue;
        }
        let Some(img) = images.get(h) else { continue };
        let Ok(rgba) = img.clone().try_into_dynamic().map(|d| d.into_rgba8()) else {
            warn!("z17 ortho {ty}_{tx} has an unsupported format");
            *sharp = Sharp::Missing;
            continue;
        };
        let (w, hh) = rgba.dimensions();
        let texture = images.add(mipmapped(w, hh, rgba.into_raw(), true, ImageAddressMode::ClampToEdge));
        mat.0 = materials.add(detail_material(texture, tx, ty, &coarse.grain));
        *sharp = Sharp::On; // drops the jpg handle, freeing the decoded CPU copy
    }
}

fn apply_vscale(vs: Res<VScale>, mut chunks: Query<&mut Transform, With<Chunk>>) {
    if vs.is_changed() {
        for mut tf in &mut chunks {
            tf.scale.y = vs.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_follows_the_mesh_triangles() {
        let coarse: Vec<Vec<f32>> = (0..NT * NT)
            .map(|_| (0..COARSE_VERTS * COARSE_VERTS).map(|k| ((k * 7919) % 13) as f32).collect())
            .collect();
        let s = Surface { coarse: coarse.clone(), detail: vec![None; NT * NT] };
        let sp = TILE_M / (COARSE_VERTS - 1) as f32;
        let (x0, z0) = chunk_origin(3, 5);
        let at = |i: usize, j: usize| coarse[5 * NT + 3][j * COARSE_VERTS + i];
        assert!((s.height(x0 + 2.0 * sp, z0 + 4.0 * sp) - at(2, 4)).abs() < 1e-3);
        // The quad centre lies on the v10-v01 diagonal: their mean, not the 4-corner bilinear mean.
        let mid = s.height(x0 + 2.5 * sp, z0 + 4.5 * sp);
        assert!((mid - (at(3, 4) + at(2, 5)) / 2.0).abs() < 1e-3, "{mid}");
    }

    #[test]
    fn mip_chain_is_complete_and_averaged() {
        let img = mipmapped(4, 2, [[0u8, 0, 0, 255], [200, 100, 40, 255]].repeat(4).concat(), false, ImageAddressMode::ClampToEdge);
        assert_eq!(img.texture_descriptor.mip_level_count, 3); // 4x2, 2x1, 1x1
        let data = img.data.unwrap();
        assert_eq!(data.len(), (8 + 2 + 1) * 4);
        assert_eq!(&data[data.len() - 4..], &[100, 50, 20, 255]);
        // sRGB: black + white average to 50 % linear light = sRGB 188, not the gamma-space 128.
        let img = mipmapped(2, 1, vec![0, 0, 0, 255, 255, 255, 255, 255], true, ImageAddressMode::ClampToEdge);
        assert_eq!(&img.data.unwrap()[8..], &[188, 188, 188, 255]);
    }

    #[test]
    fn grain_normal_map_tiles_seamlessly() {
        let img = grain_normal_map();
        assert_eq!(img.texture_descriptor.mip_level_count, 10); // 512 .. 1
        let d = img.data.unwrap();
        let px = |x: usize, y: usize, k: usize| d[(y * GRAIN_PX + x) * 4 + k] as i32;
        let (mut wrap, mut inner) = (0, 0);
        for y in 0..GRAIN_PX {
            for k in 0..3 {
                wrap = wrap.max((px(0, y, k) - px(GRAIN_PX - 1, y, k)).abs());
                for x in 0..GRAIN_PX - 1 {
                    inner = inner.max((px(x, y, k) - px(x + 1, y, k)).abs());
                }
            }
        }
        assert!(inner > 0 && wrap <= 2 * inner, "wrap {wrap} vs interior {inner}");
    }
}
