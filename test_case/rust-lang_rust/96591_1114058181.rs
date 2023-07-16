plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused imports: `AllocRange`, `ConstValue`
 --> compiler/rustc_middle/src/ty/print/pretty.rs:1:29
  |
1 | use crate::mir::interpret::{AllocRange, ConstValue, GlobalAlloc, Pointer, Provenance, Scalar};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `ConstValue`, `Scalar`
  --> compiler/rustc_middle/src/ty/context.rs:10:64
   |
10 | use crate::mir::interpret::{self, Allocation, ConstAllocation, ConstValue, Scalar};

error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 2 previous errors
