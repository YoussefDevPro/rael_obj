use rael::Color;

/// The kinds of lights that can be used in the scene
#[derive(Clone, Copy, Debug)]
pub enum LightKind {
    /// A directional light source, which has a direction but no position.
    /// This is like the sun.
    Directional {
        direction: (f32, f32, f32),
    },
    /// A point light source, which has a position and intensity.
    Point {
        position: (f32, f32, f32),
        intensity: f32,
    },
    /// An ambient light source, which illuminates all surfaces equally.
    Ambient {
        intensity: f32,
    },
}

/// A light source in the scene
#[derive(Clone, Copy, Debug)]
pub struct Light {
    /// The kind of light
    pub kind: LightKind,
    /// The color of the light
    pub color: Color,
}

/// Normalizes a 3D vector.
fn normalize(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    if len == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    (v.0 / len, v.1 / len, v.2 / len)
}

/// Computes the dot product of two 3D vectors.
fn dot(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

/// Clamps a value between 0 and 1.
fn clamp01(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

/// Computes the color of a fragment based on the lighting in the scene.
///
/// This function implements a simple diffuse lighting model. It iterates over all
/// the lights in the scene and computes the contribution of each light to the
/// final color of the fragment. The final color is then tonemapped to the 0-255
/// range.
pub fn compute_light(
    normal: (f32, f32, f32),
    frag_pos: (f32, f32, f32),
    lights: &[Light],
    base_color: Color,
) -> Color {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    let normal = normalize(normal);
    let base_r = base_color.r as f32 / 255.0;
    let base_g = base_color.g as f32 / 255.0;
    let base_b = base_color.b as f32 / 255.0;

    for light in lights {
        let light_r = light.color.r as f32 / 255.0;
        let light_g = light.color.g as f32 / 255.0;
        let light_b = light.color.b as f32 / 255.0;

        match light.kind {
            LightKind::Directional { direction } => {
                let dir = normalize(direction);
                let diff = clamp01(dot(normal, dir));
                r += base_r * light_r * diff;
                g += base_g * light_g * diff;
                b += base_b * light_b * diff;
            }
            LightKind::Point {
                position,
                intensity,
            } => {
                let light_dir = normalize((
                    position.0 - frag_pos.0,
                    position.1 - frag_pos.1,
                    position.2 - frag_pos.2,
                ));
                let diff = clamp01(dot(normal, light_dir)) * intensity;
                r += base_r * light_r * diff;
                g += base_g * light_g * diff;
                b += base_b * light_b * diff;
            }
            LightKind::Ambient { intensity } => {
                r += base_r * light_r * intensity;
                g += base_g * light_g * intensity;
                b += base_b * light_b * intensity;
            }
        }
    }

    // Tonemap the final color to the 0-255 range.
    // This is a simple Reinhard tonemapping operator.
    let tone_map = |x: f32| 255.0 * (x / (x + 1.0));

    Color {
        r: tone_map(r).clamp(0.0, 255.0) as u8,
        g: tone_map(g).clamp(0.0, 255.0) as u8,
        b: tone_map(b).clamp(0.0, 255.0) as u8,
    }
}