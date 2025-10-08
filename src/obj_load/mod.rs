use obj::{Obj, load_obj};
use rael::Color;
use std::fs::File;
use std::io::BufReader;
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

pub fn draw_obj(
    path: String,
    center: [f32; 3],
    scale: f32,
    rotation: [i32; 3],
    position: [f32; 3],
    width: i32,
    height: i32,
    fov: f32,
) -> Vec<(i32, i32, Color)> {
    let mut pixels = Vec::new();
    let input = BufReader::new(File::open(path).expect("AHHHHH NOOOOOOO? IT CRASHED? SEE WHY 3"));
    let model: Obj = load_obj(input).expect("AAAH? IT CRASHED WHEN TRYING TO LOAD? NOOOO 3");

    for face in model.indices.chunks(3) {
        if face.len() < 3 {
            continue; // skip incomplete triangles
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

        let p0 = project_to_screen(v0, width, height, fov);
        let p1 = project_to_screen(v1, width, height, fov);
        let p2 = project_to_screen(v2, width, height, fov);

        if let (Some(p0), Some(p1), Some(p2)) = (p0, p1, p2) {
            pixels.push((p0.0, p0.1, Color { r: 255, g: 0, b: 0 }));
            pixels.push((p1.0, p1.1, Color { r: 255, g: 0, b: 0 }));
            pixels.push((p2.0, p2.1, Color { r: 255, g: 0, b: 0 }));
        }
    }

    pixels
}
