rust
use std::num::NonZeroU64;
pub fn foo(x: u64, y: NonZeroU64) -> u64 {
    x / y.get()
}
