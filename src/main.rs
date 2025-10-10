mod obj_load;
use crossterm::{
    cursor::{Hide, Show},
    event::{Event, KeyCode, KeyEvent, poll, read},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use obj::Obj;
use obj::load_obj;
use obj_load::draw_obj;
use rael::{Canvas, Color};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::{Write, stdout};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        _ = execute!(stdout(), LeaveAlternateScreen, Show);
        _ = disable_raw_mode();
    }
}

fn main() -> std::io::Result<()> {
    let input = BufReader::new(File::open("source/blahaj_tri.obj").expect("Failed to open OBJ"));
    let model: Obj = load_obj(input).expect("Failed to load OBJ");

    let _clean_up = CleanUp;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = crossterm::terminal::size()?;

    // Mutable properties for the object
    let mut rotation = [20, 0, 0];
    let mut position = [0.0, 0.0, 1.0];
    let mut fov = 1.0;
    let scale = 2.0;
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
        );

        let mut seen = HashSet::new();
        for (x, y, color) in blahaj {
            if seen.insert((x, y)) {
                canvas.set_pixel(x as usize, y as usize, 1, color);
            }
        }
        let output = canvas.render();
        execute!(stdout, Print(output))?;
        stdout.flush()?;
    }

    Ok(())
}
