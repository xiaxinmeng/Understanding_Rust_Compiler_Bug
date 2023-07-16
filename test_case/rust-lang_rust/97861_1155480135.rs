plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused import: `TyCtxt`
  --> compiler/rustc_lint/src/types.rs:11:54
   |
11 | use rustc_middle::ty::{self, AdtKind, DefIdTree, Ty, TyCtxt, TypeFoldable, TypeSuperFoldable};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to previous error
Build completed unsuccessfully in 0:02:54
