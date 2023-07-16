plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0425]: cannot find value `try_reserve` in module `sym`
   --> compiler/rustc_feature/src/accepted.rs:298:16
    |
298 |     (accepted, try_reserve, "1.56.0", Some(48043), None),
    |                ^^^^^^^^^^^ not found in `sym`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_feature` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
