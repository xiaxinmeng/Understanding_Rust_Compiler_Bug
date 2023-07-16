rust
use crate::menu::{Sound, Volume};

internal mod color;

...

/// A type for storing text and an associated color it should
/// be drawn as.
internal struct ColoredText {
    internal color: types::Color,
    internal text: &'static str,
}
