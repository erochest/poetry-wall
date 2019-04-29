use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PoetryWallError {}

impl fmt::Display for PoetryWallError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for PoetryWallError {}
