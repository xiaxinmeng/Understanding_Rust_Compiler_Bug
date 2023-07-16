
 error: the feature `renamed_spin_loop` has been stable since 1.47.0 and no longer requires an attribute to enable
   --> library/std/src/lib.rs:215:32
    |
215 | #![cfg_attr(bootstrap, feature(renamed_spin_loop))]
    |                                ^^^^^^^^^^^^^^^^^
    |
    = note: `-D stable-features` implied by `-D warnings`

error: aborting due to previous error

error: could not compile `std`.
