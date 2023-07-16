plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.65 (/checkout/src/tools/clippy/clippy_lints)
error[E0609]: no field `re_root_empty` on type `rustc_middle::ty::context::CommonLifetimes<'_>`
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:176:82
    |
176 |                         let ty_empty_region = cx.tcx.mk_imm_ref(cx.tcx.lifetimes.re_root_empty, ty);
    |
    |
    = note: available fields are: `re_static`, `re_erased`
For more information about this error, try `rustc --explain E0609`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:39
