plain
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `AdtDef`
  --> src/tools/clippy/clippy_lints/src/significant_drop_in_scrutinee.rs:10:24
   |
10 | use rustc_middle::ty::{AdtDef, Ty, TypeAndMut};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:33
