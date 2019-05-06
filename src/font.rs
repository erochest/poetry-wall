use rusttype::{PositionedGlyph, Font};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Result;

pub type GlyphVec<'a> = Vec<PositionedGlyph<'a>>;

pub fn load_font<P: AsRef<Path>>(filename: P) -> Result<Font<'static>> {
    let mut font_file = File::open(filename)?;
    let mut buffer = Vec::new();
    font_file.read_to_end(&mut buffer)?;
    Font::from_bytes(buffer).map_err(|e| e.into())
}
