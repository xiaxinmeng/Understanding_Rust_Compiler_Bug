plain
    Checking clippy_lints v0.1.60 (/checkout/src/tools/clippy/clippy_lints)
error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:202:36
    |
202 | ...                   if can_type_implement_copy(cx.tcx, cx.param_env, ty).is_ok() {
    |                          |
    |                          expected 4 arguments
    |
note: function defined here
note: function defined here
   --> /checkout/compiler/rustc_trait_selection/src/traits/misc.rs:19:8
    |
19  | pub fn can_type_implement_copy<'tcx>(

For more information about this error, try `rustc --explain E0061`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:32
