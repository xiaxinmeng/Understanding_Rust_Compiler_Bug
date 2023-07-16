plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking hashbrown v0.11.0
    Checking addr2line v0.16.0
error[E0412]: cannot find type `FromSecsError` in module `time`
    |
602 | impl Error for time::FromSecsError {}
    |                      ^^^^^^^^^^^^^ not found in `time`
    |
    |
help: consider importing this struct
    |
19  | use core::time::FromSecsError;
    |

error: an `#[unstable]` annotation here has no effect
    |
    |
601 | #[unstable(feature = "duration_checked_float", issue = "83400")]
    |
    |
    = note: `#[deny(ineffective_unstable_trait_impl)]` on by default

For more information about this error, try `rustc --explain E0412`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:01:29
