plain
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
error[E0425]: cannot find value `lint_reasons` in module `sym`
   --> compiler/rustc_feature/src/accepted.rs:192:16
    |
192 |     (accepted, lint_reasons, "1.64.0", Some(54503), None),
    |                ^^^^^^^^^^^^ not found in `sym`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_feature` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_feature` due to previous error
