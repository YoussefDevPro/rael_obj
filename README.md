# Rael OBJ

[Crates.io](https://crates.io/crates/rael_obj) [Docs.rs](https://docs.rs/rael_obj)

A simple OBJ file loader and renderer for the terminal, designed to be used with the [rael](https://crates.io/crates/rael) canvas.

## Description

This library provides the core functionality to load a Wavefront `.obj` model and render it to a list of pixels. These pixels can then be drawn onto a `rael::Canvas` for display in the terminal.

It supports basic 3D transformations (position, rotation, scale), lighting (ambient and point lights), and texture mapping.

## Installation

Add `rael_obj` to your project's `Cargo.toml`:

```bash
cargo add rael_obj
```

## Important: Model Requirements

**Your 3D models MUST be triangulated.**

This renderer is designed to work with triangular faces only. Models that contain quadrilaterals ("quads") or other polygons with more than three vertices (n-gons) will likely not load or render correctly.

### How to Triangulate a Model

You can easily triangulate a model using 3D modeling software like Blender:
1.  Import your model into Blender.
2.  Select the object you want to triangulate.
3.  Switch to **Edit Mode** (press `Tab`).
4.  Select all faces (press `A`).
5.  Go to `Face > Triangulate Faces` (or use the shortcut `Ctrl+T`).
6.  Export the model as an `.obj` file.

## Example Usage

Here is a simple example of how to load a model and render it to the console.

```rust
use rael_obj::*;
use std::io::{BufReader, stdout, Write};
use std::fs::File;
use crossterm::{execute, style::Print};

fn main() -> std::io::Result<()> {
    // 1. Load the model and texture
    let input = BufReader::new(File::open("source/blahaj_tri.obj").expect("Failed to open OBJ"));
    let model: Obj<TexturedVertex> = load_obj(input).expect("Failed to load OBJ");
    let texture = Texture::from_file("source/texture.001.png");

    // 2. Set up the scene
    let lights = vec![
        Light {
            kind: LightKind::Ambient { intensity: 0.8 },
            color: Color { r: 255, g: 255, b: 255 },
        },
        Light {
            kind: LightKind::Point {
                position: (5.0, 8.0, -1.0),
                intensity: 3.0,
            },
            color: Color { r: 255, g: 180, b: 180 },
        },
    ];

    // 3. Define model transformations
    let rotation = [0, 180, 0];
    let position = [0.0, 0.0, 1.5];
    let scale = 0.2;
    let center = [0.0, 0.0, 0.0];
    let fov = -4.0;

    // 4. Create a canvas to draw on
    let (width, height) = (80, 40);
    let mut canvas = Canvas::new(width, height, Color { r: 0, g: 0, b: 0 });

    // 5. Call draw_obj to get the rendered pixels
    let pixels = draw_obj(
        &model,
        center,
        scale,
        rotation,
        position,
        &canvas,
        fov,
        &lights,
        &texture,
    );

    // 6. Draw the pixels to the canvas
    for (x, y, color) in pixels {
        canvas.set_pixel(x as usize, y as usize, 1, color);
    }

    // 7. Print the canvas to the terminal
    let output = canvas.render();
    execute!(stdout(), Print(output))?;
    stdout().flush()
}
```

## License

This project is not yet licensed.
