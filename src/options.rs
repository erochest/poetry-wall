use std::path::{Path, PathBuf};

use palette::Srgb;

#[derive(Debug)]
pub struct PoetryWallOptions {
    pub poem_file: PathBuf,
    pub font_file: PathBuf,
    pub output_file: PathBuf,
    pub color: Srgb<u8>,
    pub background: Srgb<u8>,
}

impl PoetryWallOptions {
    pub fn new<P: AsRef<Path>>(
        poem_file: P,
        font_file: P,
        output_file: P,
        color: Srgb<u8>,
        background: Srgb<u8>,
    ) -> Self {
        PoetryWallOptions {
            poem_file: poem_file.as_ref().into(),
            font_file: font_file.as_ref().into(),
            output_file: output_file.as_ref().into(),
            color,
            background,
        }
    }
}
