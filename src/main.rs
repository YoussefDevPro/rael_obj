mod obj_load;
use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use obj::load_obj;
use obj::{Obj, TexturedVertex};
use obj_load::draw_obj;
use obj_load::light::{Light, LightKind};
use obj_load::texture::Texture;
use rael::{Canvas, Color};
use std::fs::File;
use std::io::BufReader;
use std::io::{stdout, Write};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        _ = execute!(stdout(), LeaveAlternateScreen, Show);
        _ = disable_raw_mode();
    }
}

fn main() -> std::io::Result<()> {
    // DATA NEEDED FOR DA FUNCTION
    let input = BufReader::new(File::open("source/blahaj_tri.obj").expect("Failed to open OBJ"));
    let model: Obj<TexturedVertex> = load_obj(input).expect("Failed to load OBJ");

    let lights = vec![
        Light {
            kind: LightKind::Ambient { intensity: 0.15 },
            color: Color {
                r: 255,
                g: 255,
                b: 255,
            },
        },
        Light {
            kind: LightKind::Point {
                position: (5.0, 8.0, 1.0),
                intensity: 2.0,
            },
            color: Color {
                r: 255,
                g: 180,
                b: 180,
            },
        },
    ];
    let texture = Texture::from_file("source/texture.001.png");

    let _clean_up = CleanUp;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = crossterm::terminal::size()?;

    // Mutable properties for the object
    let mut rotation = [0, 0, 0];
    let mut position = [0.0, 0.0, 1.0];
    let mut fov = -4.0;
    let scale = 0.2;
    let center = [0.0, 0.0, 0.0];

    loop {
        if poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Char('q') => break,
                    // Position ↓↓↓↓↓↓↓↓↓↓↓↓
                    KeyCode::Char('w') => position[2] += 0.1,
                    KeyCode::Char('s') => position[2] -= 0.1,
                    KeyCode::Char('a') => position[0] -= 0.1,
                    KeyCode::Char('d') => position[0] += 0.1,
                    KeyCode::Char('r') => position[1] -= 0.1,
                    KeyCode::Char('f') => position[1] += 0.1,

                    // Rotation ↓↓↓↓↓↓↓↓↓↓
                    KeyCode::Up => rotation[0] += 5,
                    KeyCode::Down => rotation[0] -= 5,
                    KeyCode::Left => rotation[1] += 5,
                    KeyCode::Right => rotation[1] -= 5,
                    KeyCode::Char('e') => rotation[2] += 5,
                    KeyCode::Char('z') => rotation[2] -= 5,

                    // FOV ↓↓↓↓↓↓
                    KeyCode::Char('=') | KeyCode::Char('+') => fov += 1.0,
                    KeyCode::Char('-') => fov -= 1.0,
                    _ => {}
                }
            }
        }

        let mut canvas = Canvas::new(width as usize, height as usize, Color { r: 0, g: 0, b: 0 });

        let blahaj = draw_obj(
            &model,
            center,
            scale,
            rotation,
            position,
            width.into(),
            height.into(),
            fov,
            &lights,
            &texture,
        );

        for (x, y, color) in blahaj {
            canvas.set_pixel(x as usize, y as usize, 1, color);
        }
        let output = canvas.render();
        execute!(stdout, Print(output))?;
        stdout.flush()?;
    }

    Ok(())
}
