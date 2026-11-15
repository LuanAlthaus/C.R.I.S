//! Fine relief read from the ortho image (photoclinometry), for the chunks next to the camera.
//!
//! The HiRISE DTM holds no more shape than the 4 m mesh already shows (its 1 m posts differ from it by
//! 3–6 cm RMS), but the z17 image resolves ripples and blocks a few metres across. The mosaic is lit
//! from the west (azimuth ≈275°, fitted against the DTM): ground that is brighter than its surroundings
//! faces the sun, so it rises towards the east. Integrating that slope from west to east gives a height.
//!
//! This is a visual estimate, not a measurement. It follows the DTM only loosely (r ≈ 0.26 at the
//! 6–30 m scales both resolve) and reads albedo changes as slope. Route planning keeps using the DTM
//! layers; `rover::tests` checks that the routes still hold on the ground this adds.

/// Vertices per side of a chunk carrying relief: 2 m spacing, twice the detail mesh.
pub const VERTS: usize = 501;
pub const SPACING_M: f32 = 2.0;
/// Eastward slope per unit of relative brightness. A Lambert surface under the mosaic's sun gives ~0.6;
/// shading explains only a third or so of the brightness variation, so the estimate is shrunk to match.
const SLOPE_PER_BRIGHTNESS: f32 = 0.2;
const MAX_SLOPE: f32 = 0.25;
const MAX_RELIEF_M: f32 = 0.6;
/// Half width (vertices) of the local mean that separates relief from albedo and removes drift: only
/// scales below ~15 m come from the image, the DTM owns the rest.
const WINDOW: usize = 3;
/// The integration forgets over this distance, which keeps its error from piling up along a row.
const LEAK_M: f32 = 8.0;
/// Relief fades to zero over this distance from the chunk edge so neighbouring chunks meet.
const EDGE_M: f32 = 8.0;
const STREAK_PASSES: usize = 2;

pub fn srgb_to_linear() -> [f32; 256] {
    std::array::from_fn(|i| {
        let c = i as f32 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    })
}

/// Box blur of half width `r` on an `n`×`n` grid (edges clamped), applied twice: a smooth local mean.
fn local_mean(a: &[f32], n: usize, r: usize) -> Vec<f32> {
    let pass = |src: &[f32], rows: bool| {
        let idx = |line: usize, k: isize| {
            let k = k.clamp(0, n as isize - 1) as usize;
            if rows { line * n + k } else { k * n + line }
        };
        let mut out = vec![0.0; n * n];
        let r = r as isize;
        for line in 0..n {
            let mut sum: f32 = (-r..=r).map(|k| src[idx(line, k)]).sum();
            for k in 0..n as isize {
                out[idx(line, k)] = sum / (2 * r + 1) as f32;
                sum += src[idx(line, k + r + 1)] - src[idx(line, k - r)];
            }
        }
        out
    };
    let mut a = a.to_vec();
    for _ in 0..2 {
        a = pass(&pass(&a, true), false);
    }
    a
}

/// Relief (m) on the `VERTS`² grid of a 1 km chunk from its `w`×`h` RGBA ortho (row 0 north, column 0
/// west; vertex index = row * VERTS + column). Zero along the chunk border.
pub fn from_ortho(rgba: &[u8], w: usize, h: usize) -> Vec<f32> {
    let n = VERTS;
    let lut = srgb_to_linear();
    // Linear luminance averaged over the ~2 m around each vertex.
    let span = |c: usize, len: usize| {
        let half = (len / (n - 1) / 2).max(1);
        (c.saturating_sub(half), (c + half).min(len))
    };
    let mut lum = vec![0.0f32; n * n];
    for j in 0..n {
        let (y0, y1) = span(j * h / (n - 1), h);
        for i in 0..n {
            let (x0, x1) = span(i * w / (n - 1), w);
            let mut sum = 0.0;
            for y in y0..y1 {
                for p in rgba[(y * w + x0) * 4..(y * w + x1) * 4].chunks_exact(4) {
                    sum += 0.2126 * lut[p[0] as usize] + 0.7152 * lut[p[1] as usize] + 0.0722 * lut[p[2] as usize];
                }
            }
            lum[j * n + i] = sum / ((x1 - x0) * (y1 - y0)) as f32;
        }
    }
    let mean = local_mean(&lum, n, WINDOW);
    let decay = (-SPACING_M / LEAK_M).exp();
    let mut relief = vec![0.0f32; n * n];
    for j in 0..n {
        let (mut acc, mut prev) = (0.0, 0.0);
        for i in 0..n {
            let k = j * n + i;
            // No-data (black) areas carry no shading.
            let slope = if mean[k] < 1e-3 { 0.0 } else { SLOPE_PER_BRIGHTNESS * (lum[k] / mean[k] - 1.0) };
            let slope = slope.clamp(-MAX_SLOPE, MAX_SLOPE);
            // Trapezoid rule: a one-sided sum would shift the relief half a vertex east of the image.
            acc = acc * decay + (prev + slope) / 2.0 * SPACING_M;
            prev = slope;
            relief[k] = acc;
        }
    }
    // Each row is integrated on its own, so the errors show as east-west streaks: blend every row
    // with its neighbours.
    for _ in 0..STREAK_PASSES {
        let rows = relief.clone();
        for j in 0..n {
            let (north, south) = (j.saturating_sub(1), (j + 1).min(n - 1));
            for i in 0..n {
                relief[j * n + i] = (rows[north * n + i] + 2.0 * rows[j * n + i] + rows[south * n + i]) / 4.0;
            }
        }
    }
    let drift = local_mean(&relief, n, WINDOW);
    let fade = |i: usize| {
        let t = (i.min(n - 1 - i) as f32 * SPACING_M / EDGE_M).min(1.0);
        t * t * (3.0 - 2.0 * t)
    };
    for j in 0..n {
        for i in 0..n {
            let k = j * n + i;
            relief[k] = (relief[k] - drift[k]).clamp(-MAX_RELIEF_M, MAX_RELIEF_M) * fade(i) * fade(j);
        }
    }
    relief
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Grey RGBA image of `px`² pixels whose linear luminance is `lum(x_m, y_m)`.
    fn image(px: usize, lum: impl Fn(f32, f32) -> f32) -> Vec<u8> {
        let m_per_px = (VERTS - 1) as f32 * SPACING_M / px as f32;
        (0..px * px)
            .flat_map(|k| {
                let l = lum((k % px) as f32 * m_per_px, (k / px) as f32 * m_per_px).clamp(0.0, 1.0);
                let c = if l <= 0.003_130_8 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 };
                let v = (c * 255.0).round() as u8;
                [v, v, v, 255]
            })
            .collect()
    }

    #[test]
    fn featureless_ground_stays_flat() {
        let r = from_ortho(&image(1024, |_, _| 0.2), 1024, 1024);
        assert!(r.iter().all(|h| h.abs() < 1e-3));
        // Black no-data pixels are not read as a slope.
        let r = from_ortho(&image(1024, |x, _| if x < 500.0 { 0.0 } else { 0.2 }), 1024, 1024);
        assert!(r[250 * VERTS + 100].abs() < 1e-3);
    }

    #[test]
    fn west_lit_ridges_come_out_as_ridges() {
        // North-south ridges 12 m apart, 0.2 m high, shaded as the mosaic would: the west face
        // (rising eastward) is brighter by slope / SLOPE_PER_BRIGHTNESS.
        let (wave, amp) = (12.0f32, 0.2f32);
        let k = std::f32::consts::TAU / wave;
        let height = |x: f32| amp * (k * x).sin();
        let px = 2048;
        let r = from_ortho(&image(px, |x, _| 0.2 * (1.0 + amp * k * (k * x).cos() / SLOPE_PER_BRIGHTNESS)), px, px);
        let j = VERTS / 2;
        let cols = 50..VERTS - 50;
        let (mut hh, mut rr, mut hr) = (0.0, 0.0, 0.0);
        for i in cols {
            let (a, b) = (height(i as f32 * SPACING_M), r[j * VERTS + i]);
            hh += a * a;
            rr += b * b;
            hr += a * b;
        }
        let corr = hr / (hh * rr).sqrt();
        let gain = (rr / hh).sqrt();
        assert!(corr > 0.9, "ridges recovered with correlation {corr}");
        assert!((0.5..1.2).contains(&gain), "amplitude ratio {gain}");
    }

    #[test]
    fn relief_is_bounded_and_zero_on_the_border() {
        // Harsh speckle: the worst case for the clamps.
        let px = 1024;
        let r = from_ortho(&image(px, |x, y| if ((x / 3.0) as u32 * 7 + (y / 5.0) as u32 * 13) % 5 < 2 { 0.9 } else { 0.02 }), px, px);
        assert!(r.iter().all(|h| h.abs() <= MAX_RELIEF_M));
        let n = VERTS;
        for i in 0..n {
            for k in [i, (n - 1) * n + i, i * n, i * n + n - 1] {
                assert_eq!(r[k], 0.0);
            }
        }
    }
}
