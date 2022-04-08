extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

pub struct Config<'ttf> {
    pub window_size: (f32, f32),
    pub palette: Palette,
    pub font: Font<'ttf, 'static>,
}

pub struct Palette {
    pub default_background_color: Color,
    pub default_light_color: Color,
    pub default_dark_color: Color,
    pub blue: Color,
    pub yellow: Color,
}

pub fn initialize_config(ttf_context: &'_ Sdl2TtfContext) -> Result<Config<'_>, crate::Error> {
    Ok(Config {
        window_size: (512.0, 512.0),
        palette: Palette {
            default_dark_color: Color::RGB(122, 95, 71),
            default_light_color: Color::RGB(250, 229, 210),
            default_background_color: Color::RGB(250, 229, 210),
            blue: Color::RGB(104, 113, 143),
            yellow: Color::RGB(222, 194, 133),
        },
        font: ttf_context.load_font("assets/fonts/Amatic-Bold.ttf", 128)?,
    })
}
