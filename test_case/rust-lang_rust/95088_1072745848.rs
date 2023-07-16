plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: use of deprecated struct `std::hash::SipHasher`: use `std::collections::hash_map::DefaultHasher` instead
    |
305 |     let mut s = SipHasher::new();
    |                 ^^^^^^^^^
    |
    |
    = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated struct `std::hash::SipHasher`: use `std::collections::hash_map::DefaultHasher` instead
    |
    |
294 |     use core::hash::{Hash, SipHasher};


error: use of deprecated associated function `std::hash::SipHasher::new`: use `std::collections::hash_map::DefaultHasher` instead
    |
305 |     let mut s = SipHasher::new();
    |                            ^^^

