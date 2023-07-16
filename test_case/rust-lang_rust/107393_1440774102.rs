plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0425]: cannot find value `note2` in module `crate::fluent_generated`
   --> compiler/rustc_lint/src/lints.rs:790:8
    |
790 | #[note(note2)]
    |        ^^^^^ not found in `crate::fluent_generated`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to previous error
