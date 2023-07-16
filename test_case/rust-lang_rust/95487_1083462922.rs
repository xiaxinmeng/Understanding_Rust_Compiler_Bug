plain

error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> compiler/rustc_typeck/src/variance/mod.rs:43:11
   |
43 |         | DefKind::AssocFn
   |           ^^^^^^^ use of undeclared type `DefKind`
error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> compiler/rustc_typeck/src/variance/mod.rs:44:11
   |
44 |         | DefKind::Enum
---

error[E0433]: failed to resolve: use of undeclared type `DefKind`
  --> compiler/rustc_typeck/src/variance/mod.rs:48:11
   |
48 |         | DefKind::Ctor(..) => {}
   |           ^^^^^^^ use of undeclared type `DefKind`
[RUSTC-TIMING] rustc_plugin_impl test:false 3.623
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused import: `hir::Node`
 --> compiler/rustc_typeck/src/variance/mod.rs:6:5
 --> compiler/rustc_typeck/src/variance/mod.rs:6:5
  |
6 | use hir::Node;
  |     ^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0433`.
[RUSTC-TIMING] rustc_typeck test:false 4.773
error: could not compile `rustc_typeck` due to 8 previous errors
warning: build failed, waiting for other jobs to finish...
