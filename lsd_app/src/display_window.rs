extern crate piston_window;
extern crate std;

use self::piston_window::*;

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

        DisplayWindow { window }
    }

    pub fn get_glyphs(&self) -> Glyphs {
        let factory = self.window.factory.clone();
        Glyphs::new("assets/DejaVuSansMono.ttf", factory, TextureSettings::new()).unwrap()
    }

    pub fn draw(&mut self, buffer: &Vec<Vec<u8>>, glyphs: &mut Glyphs) -> Option<()> {
        match self.window.next() {
            Some(e) => {
                let mut offset = 0.0;
                self.window.draw_2d(&e, |c, g| {
                    clear(BACKGROUND_COLOR, g);
                    for row in buffer {
                        let transform =
                            c.transform.trans(WINDOW_OFFSET_X, WINDOW_OFFSET_Y + offset);
                        text::Text::new_color(FONT_COLOR, FONT_SIZE).draw(
                            std::str::from_utf8(row).unwrap(),
                            glyphs,
                            &c.draw_state,
                            transform,
                            g,
                        );
                        offset += ROW_OFFSET;
                    }
                });
                Some(())
            }
            None => None,
        }
    }
}
