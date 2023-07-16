plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `function_label` in module `crate::fluent_generated`
    |
    |
756 |     #[label(function_label)]
    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`
error[E0425]: cannot find value `argument_label` in module `crate::fluent_generated`
   --> compiler/rustc_lint/src/lints.rs:758:13
    |
    |
758 |     #[label(argument_label)]
    |             ^^^^^^^^^^^^^^ not found in `crate::fluent_generated`

error[E0425]: cannot find value `map_label` in module `crate::fluent_generated`
    |
    |
760 |     #[label(map_label)]
    |             ^^^^^^^^^ not found in `crate::fluent_generated`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_lint` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to 3 previous errors
