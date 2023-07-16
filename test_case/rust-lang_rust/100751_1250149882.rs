plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: unknown lint: `empty_iterator_range`
    |
    |
463 |     #![cfg_attr(bootstrap, allow(empty_iterator_range))]
    |
    |
    = note: `-D unknown-lints` implied by `-D warnings`

error: unknown lint: `empty_iterator_range`
    |
    |
464 |     #![allow(empty_iterator_range)]

error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:01:26
