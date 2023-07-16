plain
   |
19 | use rustc_hir::def_id::DefId;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`

error: unused imports: `Encodable`, `Encoder`
  --> compiler/rustc_middle/src/ty/adt.rs:14:29
   |
14 | use rustc_serialize::{self, Encodable, Encoder};

error: unused import: `ptr`
  --> compiler/rustc_middle/src/ty/adt.rs:23:11
   |
   |
23 | use std::{ptr, str};

[RUSTC-TIMING] rustc_query_system test:false 5.896
[RUSTC-TIMING] rustc_session test:false 9.030
[RUSTC-TIMING] rustc_llvm test:false 0.269
