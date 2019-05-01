use std::{fmt, io, num, result};
use std::error::Error;

use crate::error;

pub type Result<A> = result::Result<A, error::PoetryWallError>;

#[derive(Debug)]
pub enum PoetryWallError {
    InvalidMissingOption(String),
    IOError(io::Error),
    FontReadError(rusttype::Error),
    ColorError(Option<String>),
    DimensionReadError(String),
}

impl fmt::Display for PoetryWallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoetryWallError::InvalidMissingOption(option) =>
                write!(f, "Invalid/missing option: {}", &option),
            PoetryWallError::IOError(err) =>
                write!(f, "IO Error: {:?}", err),
            PoetryWallError::FontReadError(err) =>
                write!(f, "Font reading error: {:?}", err),
            PoetryWallError::ColorError(Some(color)) =>
                write!(f, "Invalid color name: {}", &color),
            PoetryWallError::ColorError(None) =>
                write!(f, "Missing color name"),
            PoetryWallError::DimensionReadError(value) =>
                write!(f, "Invalid dimensions: {}", value),
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
            PoetryWallError::ColorError(_) => "invalid/missing color name",
            PoetryWallError::DimensionReadError(_) => "invalid dimension",
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

impl From<num::ParseIntError> for PoetryWallError {
    fn from(error: num::ParseIntError) -> Self {
        PoetryWallError::DimensionReadError(format!("Unable to parse number: {}", &error))
    }
}