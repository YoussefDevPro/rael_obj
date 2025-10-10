use obj::Obj;
use rael::Color;

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
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

fn project_to_screen(v: [f32; 3], width: i32, height: i32, fov: f32) -> Option<(i32, i32)> {
    // we don't divide to zero here
    if v[2] <= 0.0 {
        return None;
    }

    let x = (v[0] / v[2]) * fov + (width as f32 / 2.0);
    let y = (v[1] / v[2]) * fov + (height as f32 / 2.0);

    Some((x as i32, y as i32))
}

//let input = BufReader::new(File::open("../source/blahaj_tri.obj").expect("REASON"));
//let model: Obj = load_obj(input).expect("REASON");
//
// Do whatever you want
//println!("vertices --------------------------------------------");
//println!("{:?}", model.vertices);
//println!("indices ---------------------------------------------");
//println!("{:?}", model.indices);

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

fn edge(p1: (i32, i32), p2: (i32, i32), p: (i32, i32)) -> i32 {
    (p.0 - p1.0) * (p2.1 - p1.1) - (p.1 - p1.1) * (p2.0 - p1.0)
}

fn fill_triangle(
    p0: (i32, i32),
    p1: (i32, i32),
    p2: (i32, i32),
    normal: (f32, f32, f32),
    light_dir: (f32, f32, f32),
    color: Color,
) -> Vec<(i32, i32, Color)> {
    let mut pixels = Vec::new();
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

    let intensity = clamp01(dot(normalize(normal), normalize(light_dir)));

    let shaded = Color {
        r: (color.r as f32 * intensity) as u8,
        g: (color.g as f32 * intensity) as u8,
        b: (color.b as f32 * intensity) as u8,
    };

    let min_x = p0.0.min(p1.0.min(p2.0));
    let max_x = p0.0.max(p1.0.max(p2.0));
    let min_y = p0.1.min(p1.1.min(p2.1));
    let max_y = p0.1.max(p1.1.max(p2.1));

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let w0 = edge(p1, p2, (x, y));
            let w1 = edge(p2, p0, (x, y));
            let w2 = edge(p0, p1, (x, y));

            if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                pixels.push((x, y, shaded));
            }
        }
    }

    pixels
}

pub fn draw_obj(
    model: &Obj,
    center: [f32; 3],
    scale: f32,
    rotation: [i32; 3],
    position: [f32; 3],
    width: i32,
    height: i32,
    fov: f32,
) -> Vec<(i32, i32, Color)> {
    let mut pixels = Vec::new();
    let light_dir = (0.0, 0.0, 1.0);

    pixels.extend(model.indices.chunks(3).flat_map(|face| {
        if face.len() < 3 {
            return Vec::new(); // skip incomplete triangles
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

        if let (Some(p0), Some(p1), Some(p2)) = (p0, p1, p2) {
            fill_triangle(p0, p1, p2, normal, light_dir, Color { r: 255, g: 0, b: 0 })
        } else {
            return Vec::new();
        }
    }));
    pixels
}
