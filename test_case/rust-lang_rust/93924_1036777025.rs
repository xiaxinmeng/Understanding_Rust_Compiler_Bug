rust
use core::fmt::Debug;

#[derive(Debug)]
pub struct Target;

#[derive(Debug)]
pub struct Source;
impl From<Source> for Target {
    fn from(_: Source) -> Self {
        Self
    }
}

/// # Examples
///
/// 