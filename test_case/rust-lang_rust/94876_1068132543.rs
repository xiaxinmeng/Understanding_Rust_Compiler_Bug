plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `ParamConst`
  --> compiler/rustc_middle/src/thir.rs:26:45
   |
26 | use rustc_middle::ty::{self, AdtDef, Const, ParamConst, Ty, UpvarSubsts, UserType};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:04
