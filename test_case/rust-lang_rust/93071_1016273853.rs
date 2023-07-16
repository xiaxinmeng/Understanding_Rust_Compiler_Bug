plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.58 (/checkout/src/tools/clippy/clippy_lints)
error: there is no argument named `snip`
   |
   |
96 | ...                   format!("{snip}.to_string()")
   |
   |
   = help: if you intended to capture `snip` from the surrounding scope, add `#![feature(format_args_capture)]` to the crate attributes
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:54
