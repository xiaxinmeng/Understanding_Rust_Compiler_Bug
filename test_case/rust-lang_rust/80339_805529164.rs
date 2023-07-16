plain
   Compiling tracing-subscriber v0.2.16
   Compiling tracing-tree v0.1.9
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find function `next_def_id` in this scope
   --> src/librustdoc/clean/inline.rs:463:29
    |
463 |                     def_id: next_def_id(did.krate),

error: unused import: `CRATE_DEF_INDEX`
 --> src/librustdoc/clean/inline.rs:9:32
  |
  |
9 | use rustc_hir::def_id::{DefId, CRATE_DEF_INDEX};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:234:13
    |
234 |             self.name,
234 |             self.name,
    |             ^^^^^^^^^
    |             |
    |             expected enum `std::option::Option`, found struct `rustc_span::Symbol`
    |             help: try using a variant of the expected enum: `calculate_doc_coverage::_::_serde::export::Some(self.name)`
    = note: expected enum `std::option::Option<rustc_span::Symbol>`
             found struct `rustc_span::Symbol`

error: aborting due to 3 previous errors
