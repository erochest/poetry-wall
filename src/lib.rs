#![feature(test)]

#[cfg(test)]
extern crate test;

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

// TODO: poem's bounding box
// TODO: font scaling
// TODO: fix glyph background
// TODO: poem's position
// TODO: refactor to use services and make more testable

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;

    let mut font_file = File::open(&options.font_file)?;
    let mut buffer = Vec::new();

    font_file.read_to_end(&mut buffer)?;
    let font = Font::from_bytes(&buffer)?;

    let scale = Scale::uniform(options.font_size);
    let color = options.color;
    let v_metrics = font.v_metrics(scale);

    let mut glyphs = Vec::new();
    let mut top = 20.0 + v_metrics.ascent;
    let line_height = v_metrics.ascent + v_metrics.descent.abs() + v_metrics.line_gap;
    let left = 20.0;
    for text in poem.0 {
        let mut line_glyphs = font.layout(&text, scale, point(left, top)).collect::<Vec<_>>();
        top += line_height;
        if top >= (options.dimensions.height as f32) {
            break;
        }
        glyphs.append(&mut line_glyphs);
    }

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
mod tests {
    use test::Bencher;

    use image::{DynamicImage, ImageBuffer, Rgba};

    static WIDTH: u32 = 2880;
    static HEIGHT: u32 = 2560;

    // bench:  31,252,308 ns/iter (+/- 4,432,156)
    // Using this one because the others really only work with a white/gray/black
    // background color. One that fills in all places in the vector with the same
    // number.
    #[bench]
    fn set_background_enumerate_pixels(b: &mut Bencher) {
        let background = [255, 255, 255, 255];
        b.iter(|| {
            let mut image = DynamicImage::new_rgba8(WIDTH, HEIGHT).to_rgba();
            for (_, _, pixel) in image.enumerate_pixels_mut() {
                pixel.data = background;
            }
        });
    }

    // 35,726,590 ns/iter (+/- 4,846,295)
    #[bench]
    fn set_background_vec_with_capacity(b: &mut Bencher) {
        let capacity = (4 * WIDTH * HEIGHT) as usize;
        b.iter(|| {
            let mut buffer = Vec::with_capacity(capacity);
            let mut i = 0;
            while i < capacity {
                buffer.push(0);
                i += 1;
            }
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> = ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
            image.map(|i| i.is_empty());
        });
    }

    // 22,013,609 ns/iter (+/- 2,043,988)
    // Is it worth special-casing this so I use this when it's white, gray, or black?
    #[bench]
    fn set_background_vec_macro(b: &mut Bencher) {
        let capacity = (4 * WIDTH * HEIGHT) as usize;
        b.iter(|| {
            let buffer = vec![0; capacity];
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> = ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
            image.map(|i| i.is_empty());
        });
    }
}