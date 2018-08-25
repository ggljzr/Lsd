extern crate piston_window;
extern crate std;

use self::piston_window::*;

use display::Display;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 100;

const WINDOW_OFFSET_X: f64 = 10.0;
const WINDOW_OFFSET_Y: f64 = 50.0;
const ROW_OFFSET: f64 = 40.0;

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.2, 1.0];

const FONT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FONT_SIZE: u32 = 32;

pub struct DisplayWindow {
    window: PistonWindow,
    glyphs: Glyphs
}

/*
Based on https://github.com/PistonDevelopers/piston-examples/blob/master/src/hello_world.rs
*/
impl DisplayWindow {
    pub fn new() -> DisplayWindow {
        let mut window: PistonWindow = WindowSettings::new("Display Window", [WIDTH, HEIGHT])
            .exit_on_esc(true)
            .build()
            .unwrap();

        window.set_lazy(false);
        //window.set_ups(30);
        window.set_max_fps(60);

        let factory = window.factory.clone();
        let glyphs = Glyphs::new("assets/DejaVuSansMono.ttf", factory, TextureSettings::new()).unwrap();

        DisplayWindow { window, glyphs}
    }

    fn _draw_rows(rows: &Vec<Vec<u8>>, glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
        let mut offset = 0.0;
        for row in rows {
            let transform = c.transform.trans(WINDOW_OFFSET_X, WINDOW_OFFSET_Y + offset);
            text::Text::new_color(FONT_COLOR, FONT_SIZE).draw(
                std::str::from_utf8(row).unwrap(),
                glyphs,
                &c.draw_state,
                transform,
                g,
            );
            offset += ROW_OFFSET;
        }
    }

    pub fn draw(&mut self, display: &Display) -> Option<()> {
        match self.window.next() {
            Some(e) => {
                let gl = &mut self.glyphs; //borrow glyphs for closure
                self.window.draw_2d(&e, |c, g| {
                    clear(BACKGROUND_COLOR, g);
                    DisplayWindow::_draw_rows(display.get_buffer(), gl, c, g);    
                });
                Some(())
            }
            None => None,
        }
    }
}
