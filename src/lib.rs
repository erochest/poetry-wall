#![feature(test)]

#[cfg(test)]
extern crate test;

use crate::error::Result;
use crate::font::load_font;
use crate::metrics::Metrics;
use crate::options::PoetryWallOptions;
use crate::poem::Poem;
use crate::image_utils::{create_image, render_glyphs};

pub mod bounding_box;
pub mod color;
pub mod dimension;
pub mod error;
pub mod font;
pub mod image_utils;
pub mod metrics;
pub mod options;
pub mod poem;

// TODO: refactor to use modules and services and make more testable

pub fn create_poetry_wall(options: &PoetryWallOptions) -> Result<()> {
    let poem = Poem::from_file(&options.poem_file)?;
    let font = load_font(&options.font_file)?;
    let metrics = Metrics::compute_metrics(options, &poem, font);
    let glyphs = metrics.create_glyphs(poem.lines());
    let background = options.background.srgb();

    let mut image = create_image(
        options.dimensions.width,
        options.dimensions.height,
        background.red.into(),
        background.green.into(),
        background.blue.into(),
    );
    render_glyphs(&mut image, &glyphs, &options.color, &options.background);

    image.save(&options.output_file)?;

    Ok(())
}