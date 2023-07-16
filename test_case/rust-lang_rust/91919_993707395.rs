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

error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
