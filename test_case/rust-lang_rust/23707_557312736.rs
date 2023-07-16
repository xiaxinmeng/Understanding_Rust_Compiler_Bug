Rust
use std::borrow::Cow;
enum Test <'a> {
    Int(u8),
    Array(Cow<'a, [Test<'a>]>),
}
