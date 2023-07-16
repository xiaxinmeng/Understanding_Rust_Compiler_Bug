
use std::fmt;

pub trait Pretty {
    fn to_pretty(&self) -> String;
}

pub struct Ooh<T>(T);

impl<T: Pretty> fmt::Display for Ooh<T> {
    /// Uses [`Pretty`] to format the inner object.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_pretty())
    }
}
