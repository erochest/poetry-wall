use std::str::FromStr;

use crate::error::PoetryWallError;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Dimension { width, height }
    }
}

impl FromStr for Dimension {
    type Err = PoetryWallError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('x').collect::<Vec<_>>();
        if parts.len() == 2 {
            Ok(Dimension::new(parts[0].parse()?, parts[1].parse()?))
        } else {
            Err(PoetryWallError::DimensionReadError(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use spectral::prelude::*;

    use crate::dimension::Dimension;

    #[test]
    fn reads_from_string() {
        let dim = Dimension::from_str("256x1024");
        assert_that(&dim).is_ok().is_equal_to(&Dimension::new(256, 1024));
    }

    #[test]
    fn fails_on_invalid_number() {
        let dim = Dimension::from_str("256xhihihi");
        assert_that(&dim).is_err();
    }

    #[test]
    fn fails_on_invalid_dimension() {
        let dim = Dimension::from_str("");
        assert_that(&dim).is_err();
    }
}