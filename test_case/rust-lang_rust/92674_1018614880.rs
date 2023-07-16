plain
    |
111 | use rustc_hir as hir;
    |     ^^^^^^^^^^^^^^^^
    |
    = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_hir::definitions::DefPathData`
   --> compiler/rustc_middle/src/mir/interpret/mod.rs:113:5
    |
113 | use rustc_hir::definitions::DefPathData;
113 | use rustc_hir::definitions::DefPathData;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `Pos`
   --> compiler/rustc_middle/src/mir/interpret/mod.rs:117:18
    |
117 | use rustc_span::{Pos, Span};

error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
