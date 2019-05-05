#![feature(test)]

#[cfg(test)]
extern crate test;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use image::{DynamicImage, ImageBuffer, Rgba};
use palette::Srgb;
use rusttype::{Font, point, PositionedGlyph};

use crate::error::Result;
use crate::metrics::Metrics;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;

pub mod dimension;
pub mod error;
pub mod metrics;
pub mod options;
pub mod poem;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
type GlyphVec<'a> = Vec<PositionedGlyph<'a>>;
type Color = Srgb<u8>;

// TODO: refactor to use modules and services and make more testable

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;
    let font = load_font(&options.font_file)?;
    let metrics = compute_metrics(options, &poem, font);
    let glyphs = create_glyphs(&metrics, &poem.0);

    let mut image = create_image(
        options.dimensions.width,
        options.dimensions.height,
        options.background.red.into(),
        options.background.green.into(),
        options.background.blue.into(),
    );
    render_glyphs(&mut image, &glyphs, &options.color, &options.background);

    image.save(&options.output_file)?;

    Ok(())
}

struct BoundingBox {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

fn load_font<P: AsRef<Path>>(filename: P) -> Result<Font<'static>> {
    let mut font_file = File::open(filename)?;
    let mut buffer = Vec::new();
    font_file.read_to_end(&mut buffer)?;
    Font::from_bytes(buffer).map_err(|e| e.into())
}

// TODO: Make a method of `Metrics`?
fn create_glyphs<'a>(metrics: &Metrics, lines: &Vec<String>) -> GlyphVec<'a> {
    let mut glyphs = Vec::new();
    let mut top = metrics.top_offset + metrics.v_metrics.ascent;
    let line_height =
        metrics.v_metrics.ascent + metrics.v_metrics.descent.abs() + metrics.v_metrics.line_gap;
    let left = metrics.left_offset;
    for text in lines {
        let mut line_glyphs = metrics
            .font
            .layout(&text, metrics.scale, point(left, top))
            .collect::<Vec<_>>();
        top += line_height;
        glyphs.append(&mut line_glyphs);
    }
    glyphs
}

// TODO: Make a ctor for `BoundingBox`?
fn compute_bounding_box(glyphs: &GlyphVec) -> BoundingBox {
    let mut bb = BoundingBox {
        top: 0,
        left: 0,
        bottom: 0,
        right: 0,
    };

    for glyph in glyphs {
        if let Some(glyph_bb) = glyph.pixel_bounding_box() {
            bb.top = bb.top.min(glyph_bb.min.y);
            bb.left = bb.left.min(glyph_bb.min.x);
            bb.bottom = bb.bottom.max(glyph_bb.max.y);
            bb.right = bb.right.max(glyph_bb.max.x);
        }
    }

    bb
}

// TODO: Make a ctor for `Metrics`?
fn compute_metrics(options: &PoetryWallOptions, poem: &Poem, font: Font<'static>) -> Metrics {
    let mut metrics = Metrics::new(font, options.font_size, 0.0, 0.0);
    let bounding_box = loop {
        let glyphs = create_glyphs(&metrics, &poem.0);
        let bb = compute_bounding_box(&glyphs);
        if ((bb.bottom - bb.top) as u32) < options.dimensions.height {
            break bb;
        }
        metrics.rescale_by(0.9);
    };
    let rendered_height = (bounding_box.bottom - bounding_box.top) as u32;
    let rendered_width = (bounding_box.right - bounding_box.left) as u32;

    metrics.top_offset = options
        .top
        .map(|v| v as f32)
        .unwrap_or_else(|| 0.33 * (options.dimensions.height - rendered_height) as f32);
    metrics.left_offset = options
        .left
        .map(|v| v as f32)
        .unwrap_or_else(|| 0.25 * (options.dimensions.width - rendered_width) as f32);

    metrics
}

// TODO: `image` utility module?
fn create_image(width: u32, height: u32, red: u8, green: u8, blue: u8) -> Image {
    let mut image = DynamicImage::new_rgba8(width, height).to_rgba();
    let background = [red, green, blue, 255];
    for (_, _, pixel) in image.enumerate_pixels_mut() {
        pixel.data = background;
    }
    image
}

// TODO: `image` utility module?
fn render_glyphs(image: &mut Image, glyphs: &GlyphVec, color: &Color, background: &Color) {
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let pixel_color = alpha_composite(color, background, v);
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba {
                        data: pixel_color,
                    },
                )
            });
        }
    }
}

// TODO: `image` utility module? Method on a `Color` newtype?
// TODO: Output a `Color` (why outputting in a different format than accepting input?)
fn alpha_composite(color: &Color, background: &Color, alpha: f32) -> [u8; 4] {
    let red = ((color.red as f32) / 255.0) * alpha + ((background.red as f32) / 255.0) * (1.0 - alpha);
    let green = ((color.green as f32) / 255.0) * alpha + ((background.green as f32) / 255.0) * (1.0 - alpha);
    let blue = ((color.blue as f32) / 255.0) * alpha + ((background.blue as f32) / 255.0) * (1.0 - alpha);
    [(red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8, 255]
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
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> =
                ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
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
            let image: Option<ImageBuffer<Rgba<_>, Vec<_>>> =
                ImageBuffer::from_vec(WIDTH, HEIGHT, buffer);
            image.map(|i| i.is_empty());
        });
    }
}