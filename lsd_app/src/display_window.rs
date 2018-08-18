extern crate piston_window;

use self::piston_window::*;

pub struct DisplayWindow {
    window: PistonWindow,
}

/*
Based on https://github.com/PistonDevelopers/piston-examples/blob/master/src/hello_world.rs
*/
impl DisplayWindow {
    pub fn new() -> DisplayWindow {
        let mut window: PistonWindow = WindowSettings::new("Display Window", [800, 100])
            .exit_on_esc(true)
            .build()
            .unwrap();

        window.set_lazy(false);
        DisplayWindow{window}
    }

    pub fn get_glyphs(&self) -> Glyphs {
        let factory = self.window.factory.clone();
        Glyphs::new("assets/DejaVuSansMono.ttf", factory, TextureSettings::new()).unwrap()
    }

    pub fn draw(&mut self, s: &str, glyphs: &mut Glyphs) -> Option<()> {
        match self.window.next() {
            Some(e) => {
                self.window.draw_2d(&e, |c, g| {
                    let transform = c.transform.trans(10.0, 50.0);
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                        s,
                        glyphs,
                        &c.draw_state,
                        transform, g
                    );
                });
                Some(())
            },
            None => None
        }

    }
}