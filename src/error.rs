use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PoetryWallError {
    InvalidMissingOption(String)
}

impl fmt::Display for PoetryWallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PoetryWallError::InvalidMissingOption(option) =>
                write!(f, "Invalid/missing option: {}", &option)
        }
    }
}

impl Error for PoetryWallError {
    fn description(&self) -> &str {
        match self {
            PoetryWallError::InvalidMissingOption(_) =>
                "invalid/missing option"
        }
    }
}
