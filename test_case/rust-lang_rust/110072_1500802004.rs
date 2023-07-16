plain

error: the feature `is_terminal` has been stable since 1.69.0-beta.1 and no longer requires an attribute to enable
   --> library/std/src/lib.rs:257:32
    |
257 | #![cfg_attr(bootstrap, feature(is_terminal))]
    |
    |
    = note: `-D stable-features` implied by `-D warnings`
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:21
