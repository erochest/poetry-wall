use std::fs::File;
use std::io::Read;

use image::{DynamicImage, Rgba};
use rusttype::{Font, point, Scale};

use crate::error::Result;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;

pub mod error;
pub mod options;
pub mod poem;

// TODO: Color setting
// TODO: Font size
// TODO: Image size
// TODO: Background color
// TODO: poem's bounding box (kerning)
// TODO: font scaling
// TODO: poem's position
// TODO: render all lines

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;

    let mut font_file = File::open(&options.font_file)?;
    let mut buffer = Vec::new();

    font_file.read_to_end(&mut buffer)?;
    let font = Font::from_bytes(&buffer)?;

    let scale = Scale::uniform(32.0);
    let color = (255, 255, 255);
    let v_metrics = font.v_metrics(scale);

    let text = &poem.0[0];
    let glyphs = font
        .layout(&text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect::<Vec<_>>();

    let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let mut image = DynamicImage::new_rgba8(width + 40, height + 40).to_rgba();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba {
                        data: [color.0, color.1, color.2, (v * 255.0) as u8],
                    },
                )
            });
        }
    }

    image.save(&options.output_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {}