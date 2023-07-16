rust
use std::num::NonZeroUsize;

pub fn foo(x: NonZeroUsize) -> NonZeroUsize {
    Some(x).unwrap()
}
