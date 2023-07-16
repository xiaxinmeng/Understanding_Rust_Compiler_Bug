
error: associated function is never used: `from_display`
   --> src/librustdoc/html/format.rs:105:14
    |
105 |     crate fn from_display<T: std::fmt::Display>(&mut self, t: T) {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
