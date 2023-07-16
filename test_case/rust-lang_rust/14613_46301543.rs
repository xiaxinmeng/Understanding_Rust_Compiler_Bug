 rust
fn from_utf16<I: Iterator<u16>>(it: I) -> FromUtf16<I> { ... }
fn to_utf16<I: Iterator<char>>(it: I) -> ToUtf16<I> { ... }

impl<I: Iterator<u16>> Iterator<char> for FromUtf16<I> { ... }
impl<I: Iterator<char>> Iterator<u16> for ToUtf16<I> { ... }
