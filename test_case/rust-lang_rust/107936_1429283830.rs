plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `TypeVisitable`
 --> compiler/rustc_middle/src/ty/instance.rs:3:84
  |
3 | use crate::ty::{self, Binder, Ty, TyCtxt, TyKind, TypeFoldable, TypeSuperFoldable, TypeVisitable};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:01:08
