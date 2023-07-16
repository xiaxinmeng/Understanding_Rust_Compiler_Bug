rust
use std::{num::ParseFloatError, str::FromStr};

/// Uses the rules from [`f64::from_str().unwrap()`].
#[derive(Debug)]
pub struct Number {
    pub value: f64,
}

impl FromStr for Number {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f64>()?;
        Ok(Self{value})
    }
}
