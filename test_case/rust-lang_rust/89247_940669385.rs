plain
  |
9 | use rustc_span::symbol::sym;
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `Symbol`
  --> src/intrinsics/mod.rs:12:35
   |
   |
12 | use rustc_span::symbol::{kw, sym, Symbol};

error[E0615]: attempted to take value of method `def_id` on type `rustc_middle::ty::Instance<'tcx>`
   --> src/intrinsics/mod.rs:410:47
    |
    |
410 |     let intrinsic = fx.tcx.item_name(instance.def_id);
    |
help: use parentheses to call the method
    |
    |
410 |     let intrinsic = fx.tcx.item_name(instance.def_id());

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustc_codegen_cranelift` due to 3 previous errors
Build completed unsuccessfully in 0:03:56
