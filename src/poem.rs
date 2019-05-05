use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct Poem(Vec<String>);

impl Poem {
    pub fn new(lines: Vec<String>) -> Self {
        Poem(lines)
    }

    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(Poem::new(buffer.lines().map(|line| line.into()).collect()))
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::error::PoetryWallError;
    use crate::poem::Poem;

    #[test]
    fn reads_from_file() {
        let poem = Poem::from_file("./tests/fixtures/fly-buzz.md");
        assert_that(&poem)
            .is_ok()
            .map(|p| &p.0)
            .has_length(21);
    }

    #[test]
    fn returns_error_missing_file() {
        let poem = Poem::from_file("./does/not/exist");
        assert_that(&poem)
            .is_err()
            .map(|err| match err {
                PoetryWallError::IOError(_) => &true,
                _ => &false,
            })
            .is_true()
    }
}