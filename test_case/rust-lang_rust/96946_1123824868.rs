plain
    Checking cranelift-frontend v0.83.0
    Checking cranelift-native v0.83.0
    Checking cranelift-object v0.83.0
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
error: no rules expected the token `transmute`
    |
29  | macro_rules! intrinsic_match {
    | ---------------------------- when calling this macro
...
...
539 |         transmute, (c from) {
    |         ^^^^^^^^^ no rules expected this token in macro call
error: unused import: `rustc_middle::ty::print::with_no_trimmed_paths`
  --> src/intrinsics/mod.rs:58:5
   |
58 | use rustc_middle::ty::print::with_no_trimmed_paths;
58 | use rustc_middle::ty::print::with_no_trimmed_paths;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `kw`
  --> src/intrinsics/mod.rs:60:26
   |
   |
60 | use rustc_span::symbol::{kw, sym, Symbol};


error: unused import: `cranelift_codegen::ir::AtomicRmwOp`
   |
   |
63 | use cranelift_codegen::ir::AtomicRmwOp;

error: could not compile `rustc_codegen_cranelift` due to 4 previous errors
Build completed unsuccessfully in 0:03:08
