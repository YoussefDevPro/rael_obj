use crate::obj_load::light::{compute_light, Light};
use obj::{Obj, TexturedVertex};
use rael::Color;
pub mod light;
pub mod texture;
use crate::Texture;

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

fn rotate(v: [f32; 3], rotation_deg: [i32; 3]) -> [f32; 3] {
    let (mut x, mut y, mut z) = (v[0], v[1], v[2]);

    let rx = deg_to_rad(rotation_deg[0] as f32);
    let ry = deg_to_rad(rotation_deg[1] as f32);
    let rz = deg_to_rad(rotation_deg[2] as f32);

    // rotate X
    let (sinx, cosx) = rx.sin_cos();
    let new_y = y * cosx - z * sinx;
    let new_z = y * sinx + z * cosx;
    y = new_y;
    z = new_z;

    // rotate Y
    let (siny, cosy) = ry.sin_cos();
    let new_x = x * cosy + z * siny;
    let new_z = -x * siny + z * cosy;
    x = new_x;
    z = new_z;

    // rotate Z
    let (sinz, cosz) = rz.sin_cos();
    let new_x = x * cosz - y * sinz;
    let new_y = x * sinz + y * cosz;
    x = new_x;
    y = new_y;

    [x, y, z]
}

fn compute_normal(
    v0: &(f32, f32, f32),
    v1: &(f32, f32, f32),
    v2: &(f32, f32, f32),
) -> (f32, f32, f32) {
    let u = (v1.0 - v0.0, v1.1 - v0.1, v1.2 - v0.2);
    let v = (v2.0 - v0.0, v2.1 - v0.1, v2.2 - v0.2);
    let n = (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    );
    let len = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt();
    (n.0 / len, n.1 / len, n.2 / len)
}

fn project_to_screen(v: [f32; 3], width: i32, height: i32, fov: f32) -> Option<(i32, i32)> {
    if v[2] <= 0.0 {
        return None;
    }

    let scale = (height as f32 / 2.0) * fov;

    let x = (v[0] / v[2]) * scale + (width as f32 / 2.0);
    let y = (v[1] * -1.0 / v[2]) * scale + (height as f32 / 2.0);

    Some((x as i32, y as i32))
}

fn transform_vertex(
    v: [f32; 3],
    center: [f32; 3],
    scale: f32,
    rotation: [i32; 3],
    position: [f32; 3],
) -> [f32; 3] {
    let mut p = [
        (v[0] - center[0]) * scale,
        (v[1] - center[1]) * scale,
        (v[2] - center[2]) * scale,
    ];

    p = rotate(p, rotation);
    [p[0] + position[0], p[1] + position[1], p[2] + position[2]]
}

fn fill_triangle(
    p0: (i32, i32),
    p1: (i32, i32),
    p2: (i32, i32),
    t0: (f32, f32),
    t1: (f32, f32),
    t2: (f32, f32),
    normal: (f32, f32, f32),
    v0: [f32; 3],
    v1: [f32; 3],
    v2: [f32; 3],
    texture: &Texture,
    lights: &[Light],
    depth_buffer: &mut [f32],
    width: i32,
) -> Vec<(i32, i32, Color)> {
    let mut pixels = Vec::new();

    let min_x = p0.0.min(p1.0.min(p2.0));
    let max_x = p0.0.max(p1.0.max(p2.0));
    let min_y = p0.1.min(p1.1.min(p2.1));
    let max_y = p0.1.max(p1.1.max(p2.1));

    fn edge(p1: (i32, i32), p2: (i32, i32), p: (i32, i32)) -> f32 {
        ((p.0 - p1.0) * (p2.1 - p1.1) - (p.1 - p1.1) * (p2.0 - p1.0)) as f32
    }

    let area = edge(p0, p1, p2);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x, y);
            let w0 = edge(p1, p2, p);
            let w1 = edge(p2, p0, p);
            let w2 = edge(p0, p1, p);

            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                let b0 = w0 / area;
                let b1 = w1 / area;
                let b2 = w2 / area;

                let u = b0 * t0.0 + b1 * t1.0 + b2 * t2.0;
                let v = b0 * t0.1 + b1 * t1.1 + b2 * t2.1;
                let z = b0 * v0[2] + b1 * v1[2] + b2 * v2[2];

                let idx = (y * width + x) as usize;
                if idx < depth_buffer.len() && z < depth_buffer[idx] {
                    depth_buffer[idx] = z;

                    let (r, g, b) = texture.sample(u, v);
                    let shaded = compute_light(normal, v0.into(), lights, Color { r, g, b });
                    pixels.push((x, y, shaded));
                }
            }
        }
    }

    pixels
}

pub fn draw_obj(
    model: &Obj<TexturedVertex>,
    center: [f32; 3],
    scale: f32,
    rotation: [i32; 3],
    position: [f32; 3],
    width: i32,
    height: i32,
    fov: f32,
    lights: &[Light],
    texture: &Texture,
) -> Vec<(i32, i32, Color)> {
    let mut pixels = Vec::new();
    let mut depth_buffer = vec![f32::INFINITY; (width * height) as usize];

    for face in model.indices.chunks(3) {
        if face.len() < 3 {
            continue;
        }

        let v0 = transform_vertex(
            model.vertices[face[0] as usize].position,
            center,
            scale,
            rotation,
            position,
        );
        let v1 = transform_vertex(
            model.vertices[face[1] as usize].position,
            center,
            scale,
            rotation,
            position,
        );
        let v2 = transform_vertex(
            model.vertices[face[2] as usize].position,
            center,
            scale,
            rotation,
            position,
        );

        let normal = compute_normal(&v0.into(), &v1.into(), &v2.into());

        let p0 = project_to_screen(v0, width, height, fov);
        let p1 = project_to_screen(v1, width, height, fov);
        let p2 = project_to_screen(v2, width, height, fov);

        let t0 = model.vertices[face[0] as usize].texture;
        let t1 = model.vertices[face[1] as usize].texture;
        let t2 = model.vertices[face[2] as usize].texture;

        if let (Some(p0), Some(p1), Some(p2)) = (p0, p1, p2) {
            pixels.extend(fill_triangle(
                p0,
                p1,
                p2,
                (t0[0], t0[1]),
                (t1[0], t1[1]),
                (t2[0], t2[1]),
                normal,
                v0,
                v1,
                v2,
                texture,
                lights,
                &mut depth_buffer,
                width,
            ));
        }
    }

    pixels
}
