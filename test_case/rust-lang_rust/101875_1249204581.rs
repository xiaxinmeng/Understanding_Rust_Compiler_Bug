plain
    |
200 | ...                   if can_type_implement_copy(
    |                          ^^^^^^^^^^^^^^^^^^^^^^^
...
204 | ...                       traits::ObligationCause::dummy_with_span(span),
    |                           ---------------------------------------------- an argument of type `rustc_ast::ImplPolarity` is missing
note: function defined here
   --> /checkout/compiler/rustc_trait_selection/src/traits/misc.rs:20:8
    |
    |
20  | pub fn can_type_implement_copy<'tcx>(
help: provide the argument
    |
    |
200 |                                 if can_type_implement_copy(cx.tcx, cx.param_env, ty, /* rustc_ast::ImplPolarity */, traits::ObligationCause::dummy_with_span(span)).is_ok() {

For more information about this error, try `rustc --explain E0061`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:03
