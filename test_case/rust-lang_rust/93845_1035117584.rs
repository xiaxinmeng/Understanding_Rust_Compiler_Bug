plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0425]: cannot find value `in_band_lifetimes` in module `sym`
   --> compiler/rustc_feature/src/removed.rs:108:15
    |
108 |     (removed, in_band_lifetimes, "1.23.0", Some(44524), None,
    |               ^^^^^^^^^^^^^^^^^ not found in `sym`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_feature` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
