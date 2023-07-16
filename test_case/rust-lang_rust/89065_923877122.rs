rust
use std::num::NonZeroU64;

pub fn bax(x: u64) -> NonZeroU64 {
    NonZeroU64::MIN.saturating_add(x)
}
