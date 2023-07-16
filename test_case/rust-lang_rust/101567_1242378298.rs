rust
use std::num::NonZeroU64;

pub enum Provenance2 {
    Concrete {
        alloc_id: NonZeroU64,
        sb: u64,
    },
    Wildcard, // .0 = 0, .1 = 0
    None, // .0 = 0, .1 = 1
}
