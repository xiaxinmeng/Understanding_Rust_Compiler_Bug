rust
// Here `crate` means the root of this crate.
use crate::menu::{Sound, Volume};

// Here, `crate` means: export crate::game::color
// The `crate` is referring to `color`, not the root.
crate mod color;

...

/// A type for storing text and an associated color it should
/// be drawn as.
// Same potential confusion as `crate mod color`
crate struct ColoredText {
    crate color: types::Color,
    crate text: &'static str,
}
