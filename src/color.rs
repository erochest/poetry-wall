use std::str::FromStr;

use palette::rgb::Rgb;
use palette::Srgb;

use crate::error::PoetryWallError;

#[derive(Debug)]
pub struct Color(Srgb<u8>);

impl Color {
    pub fn new(data: Srgb<u8>) -> Self {
        Color(data)
    }

    pub fn srgb(&self) -> Srgb<u8> {
        self.0
    }

    pub fn alpha_composite(&self, background: &Color, alpha: f32) -> Color {
        let color = self.srgb();
        let background = background.srgb();

        let red = ((color.red as f32) / 255.0) * alpha + ((background.red as f32) / 255.0) * (1.0 - alpha);
        let green = ((color.green as f32) / 255.0) * alpha + ((background.green as f32) / 255.0) * (1.0 - alpha);
        let blue = ((color.blue as f32) / 255.0) * alpha + ((background.blue as f32) / 255.0) * (1.0 - alpha);

        Color::new(Rgb::new((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8))
    }

    pub fn as_array(&self) -> [u8; 4] {
        let color = self.srgb();
        [color.red as u8, color.green, color.blue, 255]
    }
}

impl FromStr for Color {
    type Err = PoetryWallError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        palette::named::from_str(s)
            .map(Color)
            .ok_or_else(|| PoetryWallError::ColorError(Some(s.into())))
    }
}
