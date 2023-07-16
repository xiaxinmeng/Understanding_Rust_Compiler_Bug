plain
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/ty.rs:131:5
    |
121 | ) -> bool {
    |      ---- expected `bool` because of return type
...
131 |     cx.tcx.type_implements_trait((trait_id, ty, ty_params, cx.param_env))
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found enum `EvaluationResult`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_utils`
