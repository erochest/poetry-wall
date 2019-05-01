use std::fs::File;
use std::io::Read;

use image::{DynamicImage, Rgba};
use rusttype::{Font, point, Scale};

use crate::error::Result;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;

pub mod dimension;
pub mod error;
pub mod options;
pub mod poem;

// TODO: speed up with larger images
// TODO: all lines
// TODO: poem's bounding box (kerning)
// TODO: font scaling
// TODO: fix glyph background
// TODO: poem's position
// TODO: render all lines

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;

    let mut font_file = File::open(&options.font_file)?;
    let mut buffer = Vec::new();

    font_file.read_to_end(&mut buffer)?;
    let font = Font::from_bytes(&buffer)?;

    let scale = Scale::uniform(options.font_size);
    let color = options.color;
    let v_metrics = font.v_metrics(scale);

    let text = &poem.0[0];
    let glyphs = font
        .layout(&text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect::<Vec<_>>();

    let mut image = DynamicImage::new_rgba8(options.dimensions.width, options.dimensions.height).to_rgba();
    let background = [options.background.red, options.background.green, options.background.blue, 255];
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.data = background;
    }
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba {
                        data: [color.red, color.green, color.blue, (v * 255.0) as u8],
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