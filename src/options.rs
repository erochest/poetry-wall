use std::path::{Path, PathBuf};

use crate::color::Color;
use crate::dimension::Dimension;

#[derive(Debug)]
pub struct PoetryWallOptions {
    pub poem_file: PathBuf,
    pub font_file: PathBuf,
    pub font_size: f32,
    pub color: Color,
    pub background: Color,
    pub dimensions: Dimension,
    pub top: Option<u32>,
    pub left: Option<u32>,
    pub output_file: PathBuf,
}

impl PoetryWallOptions {
    pub fn new<P: AsRef<Path>>(
        poem_file: P,
        font_file: P,
        font_size: f32,
        color: Color,
        background: Color,
        dimensions: Dimension,
        top: Option<u32>,
        left: Option<u32>,
        output_file: P,
    ) -> Self {
        PoetryWallOptions {
            poem_file: poem_file.as_ref().into(),
            font_file: font_file.as_ref().into(),
            font_size,
            color,
            background,
            dimensions,
            left,
            top,
            output_file: output_file.as_ref().into(),
        }
    }
}
