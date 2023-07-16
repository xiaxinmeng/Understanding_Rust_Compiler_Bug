Rust
trait FooImpl<const IS_ZERO: bool>{}

impl FooImpl<{0u8==0u8}> for () {}
