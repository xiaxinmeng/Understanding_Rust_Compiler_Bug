plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1662:57
     |
1662 | ...                   pred.self_ty().is_param(index)
     |                                               ^^^^^ expected `u32`, found `usize`
     |
help: you can convert a `usize` to a `u32` and panic if the converted value doesn't fit
     |
1662 |                                 pred.self_ty().is_param(index.try_into().unwrap())

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
