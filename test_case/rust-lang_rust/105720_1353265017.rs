plain
   Compiling rustc_hir_analysis v0.0.0 (/checkout/compiler/rustc_hir_analysis)
error[E0532]: expected tuple struct or tuple variant, found unit variant `ty::Opaque`
   --> compiler/rustc_hir_analysis/src/check/compare_method.rs:339:13
    |
339 |             ty::Opaque(..) => {
    |
   ::: /checkout/compiler/rustc_type_ir/src/sty.rs:42:5
    |
42  |     Opaque,
---
    |
1   | use rustc_infer::infer::region_constraints::GenericKind::Opaque;
    |
      and 2 other candidates
help: if you import `Opaque`, refer to it directly
    |
339 -             ty::Opaque(..) => {
339 +             Opaque(..) => {

For more information about this error, try `rustc --explain E0532`.
error: could not compile `rustc_hir_analysis` due to previous error
warning: build failed, waiting for other jobs to finish...
