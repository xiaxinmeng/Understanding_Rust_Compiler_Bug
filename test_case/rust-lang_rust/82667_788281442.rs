rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IntervalError {
    InvalidRange,
}
impl fmt::Display for IntervalError {
    fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Error for IntervalError {}

#[derive(Debug)]
pub enum ModelError {
    InvalidInterval(IntervalError),
}

impl ModelError {
    pub fn description(&self) -> &str {
        match *self {
            ModelError::InvalidInterval(ref err) => match err {
                ref otherwise => otherwise.description(),
            },
        }
    }
}
