#![feature(test)]

#[cfg(test)]
extern crate test;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use image::{DynamicImage, ImageBuffer, Rgba};
use palette::Srgb;
use rusttype::{Font, point, PositionedGlyph, Scale, VMetrics};

use crate::error::Result;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;

pub mod dimension;
pub mod error;
pub mod options;
pub mod poem;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
type GlyphVec<'a> = Vec<PositionedGlyph<'a>>;

// TODO: poem's bounding box
// TODO: font scaling
// TODO: fix glyph background
// TODO: poem's position
// TODO: refactor to use services and make more testable

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;
    let font = load_font(&options.font_file)?;

    let scale = Scale::uniform(options.font_size);
    let v_metrics = font.v_metrics(scale);

    let glyphs = create_glyphs(&poem.0, &font, scale, &v_metrics, options.dimensions.height as f32);
    let mut image = create_image(
        options.dimensions.width,
        options.dimensions.height,
        options.background.red.into(),
        options.background.green.into(),
        options.background.blue.into(),
    );
    render_glyphs(&mut image, &glyphs, &options.color);

    image.save(&options.output_file)?;

    Ok(())
}

fn load_font<P: AsRef<Path>>(filename: P) -> Result<Font<'static>> {
    let mut font_file = File::open(filename)?;
    let mut buffer = Vec::new();
    font_file.read_to_end(&mut buffer)?;
    Font::from_bytes(buffer).map_err(|e| e.into())
}

fn create_glyphs<'a>(lines: &Vec<String>, font: &'a Font, scale: Scale, v_metrics: &VMetrics, max_height: f32) -> GlyphVec<'a> {
    let mut glyphs = Vec::new();
    let mut top = 20.0 + v_metrics.ascent;
    let line_height = v_metrics.ascent + v_metrics.descent.abs() + v_metrics.line_gap;
    let left = 20.0;
    for text in lines {
        let mut line_glyphs = font.layout(&text, scale, point(left, top)).collect::<Vec<_>>();
        top += line_height;
        if top >= (max_height as f32) {
            break;
        }
        glyphs.append(&mut line_glyphs);
    }
    glyphs
}

fn create_image(width: u32, height: u32, red: u8, green: u8, blue: u8) -> Image {
    let mut image = DynamicImage::new_rgba8(width, height).to_rgba();
    let background = [red, green, blue, 255];
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.data = background;
    }
    image
}

fn render_glyphs(image: &mut Image, glyphs: &GlyphVec, color: &Srgb<u8>) {
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