plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0425]: cannot find value `lifetime_appears_in_type` in module `rustc_errors::fluent::borrowck`
    |
    |
193 |     #[label(borrowck::lifetime_appears_in_type)]
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `lifetime_appears_in_type_of`
   ::: /checkout/compiler/rustc_error_messages/src/lib.rs:37:1
    |
37  | fluent_messages! {
37  | fluent_messages! {
    | ---------------- similarly named constant `lifetime_appears_in_type_of` defined here
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to previous error
