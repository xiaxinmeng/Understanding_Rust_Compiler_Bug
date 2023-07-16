rust
use crate::menu::{Sound, Volume};

intern mod color;

...

/// A type for storing text and an associated color it should
/// be drawn as.
intern struct ColoredText {
    intern color: types::Color,
    intern text: &'static str,
}
