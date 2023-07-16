plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.4.3
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: associated function is never used: `from_display`
   --> src/librustdoc/html/format.rs:105:14
    |
105 |     crate fn from_display<T: std::fmt::Display>(&mut self, t: T) {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

