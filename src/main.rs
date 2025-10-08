mod obj_load;
use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use obj_load::draw_obj;
use rael::{Canvas, Color};
use std::collections::HashSet;
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
    let _clean_up = CleanUp;
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = crossterm::terminal::size()?;
    let mut canvas = Canvas::new(width as usize, height as usize, Color { r: 0, g: 0, b: 0 });
    // DO SHIT
    let blahaj = draw_obj(
        "source/blahaj_tri.obj".to_string(),
        [0.0, 0.0, 0.0],
        3.0,
        [30, 40, 0],
        [10.0, 0.0, 0.0],
        width.into(),
        height.into(),
        10.0,
    );

    loop {
        if poll(Duration::from_millis(0))?
            && matches!(
                read()?,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                })
            )
        {
            break;
        }

        let mut seen = HashSet::new();
        for (x, y, color) in blahaj.clone() {
            if x >= 0 && y >= 0 && x < width as i32 && y < height as i32 {
                if seen.insert((x, y)) {
                    canvas.set_pixel(x as usize, y as usize, 1, color);
                }
            }
        }
        let output = canvas.render();
        execute!(stdout, Print(output))?;
        stdout.flush()?;
    }

    Ok(())
}
