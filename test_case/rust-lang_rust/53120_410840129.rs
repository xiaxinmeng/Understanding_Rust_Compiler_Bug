rust
use crate::menu::{Sound, Volume};

crate mod color;

...

/// A type for storing text and an associated color it should
/// be drawn as.
crate struct ColoredText {
    crate color: types::Color,
    crate text: &'static str,
}
