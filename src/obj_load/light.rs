use rael::Color;

#[derive(Clone, Copy)]
pub enum LightKind {
    Directional {
        direction: (f32, f32, f32),
    },
    Point {
        position: (f32, f32, f32),
        intensity: f32,
    },
    Ambient {
        intensity: f32,
    },
}

#[derive(Clone, Copy)]
pub struct Light {
    pub kind: LightKind,
    pub color: Color,
}

fn normalize(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let len = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
    if len == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    (v.0 / len, v.1 / len, v.2 / len)
}

fn dot(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn clamp01(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

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

    for light in lights {
        match light.kind {
            LightKind::Directional { direction } => {
                let dir = normalize(direction);
                let diff = clamp01(dot(normal, dir));
                r += (light.color.r as f32 / 255.0) * diff;
                g += (light.color.g as f32 / 255.0) * diff;
                b += (light.color.b as f32 / 255.0) * diff;
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
                r += (light.color.r as f32 / 255.0) * diff;
                g += (light.color.g as f32 / 255.0) * diff;
                b += (light.color.b as f32 / 255.0) * diff;
            }
            LightKind::Ambient { intensity } => {
                r += (light.color.r as f32 / 255.0) * intensity;
                g += (light.color.g as f32 / 255.0) * intensity;
                b += (light.color.b as f32 / 255.0) * intensity;
            }
        }
    }

    r = (base_color.r as f32 / 255.0) * r;
    g = (base_color.g as f32 / 255.0) * g;
    b = (base_color.b as f32 / 255.0) * b;

    let tone_map = |x: f32| 255.0 * (x / (x + 1.0));

    Color {
        r: tone_map(r).clamp(0.0, 255.0) as u8,
        g: tone_map(g).clamp(0.0, 255.0) as u8,
        b: tone_map(b).clamp(0.0, 255.0) as u8,
    }
}
