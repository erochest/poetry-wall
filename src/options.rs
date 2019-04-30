use std::path::{PathBuf, Path};

#[derive(Debug)]
pub struct PoetryWallOptions {
    poem_file: PathBuf,
    font_file: PathBuf,
    output_file: PathBuf,
}

impl PoetryWallOptions {
    pub fn new<P: AsRef<Path>>(poem_file: P, font_file: P, output_file: P) -> Self {
        PoetryWallOptions {
            poem_file: poem_file.as_ref().into(),
            font_file: font_file.as_ref().into(),
            output_file: output_file.as_ref().into(),
        }
    }
}
