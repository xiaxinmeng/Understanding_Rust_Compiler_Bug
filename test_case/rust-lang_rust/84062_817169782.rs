plain
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/lib.rs:587:25
    |
587 |         .map_or(false, |(entry_fn_def_id, _)| def_id == entry_fn_def_id.to_def_id())
    |                         |                  |
    |                         |                  expected due to this
    |                         |                  expected due to this
    |                         expected struct `EntryFn`, found tuple
    |
    = note: expected struct `EntryFn`
                found tuple `(_, _)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_utils`
