use std::error::Error;
use std::{fmt, result, io};
use crate::error;

pub type Result<A> = result::Result<A, error::PoetryWallError>;

#[derive(Debug)]
pub enum PoetryWallError {
    InvalidMissingOption(String),
    IOError(io::Error),
    FontReadError(rusttype::Error),
}

impl fmt::Display for PoetryWallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoetryWallError::InvalidMissingOption(option) =>
                write!(f, "Invalid/missing option: {}", &option),
            PoetryWallError::IOError(err) =>
                write!(f, "IO Error: {:?}", err),
            PoetryWallError::FontReadError(err) =>
                write!(f, "Font reading error: {:?}", err)
        }
    }
}

impl Error for PoetryWallError {
    fn description(&self) -> &str {
        match self {
            PoetryWallError::InvalidMissingOption(_) =>
                "invalid/missing option",
            PoetryWallError::IOError(err) => err.description(),
            PoetryWallError::FontReadError(err) => err.description(),
        }
    }
}

impl From<io::Error> for PoetryWallError {
    fn from(io_error: io::Error) -> Self {
        PoetryWallError::IOError(io_error)
    }
}

impl From<rusttype::Error> for PoetryWallError {
    fn from(err: rusttype::Error) -> Self {
        PoetryWallError::FontReadError(err)
    }
}