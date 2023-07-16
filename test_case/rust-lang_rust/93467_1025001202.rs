plain
[RUSTC-TIMING] tracing test:false 0.581
error: unused import: `HashStableEq`
  --> compiler/rustc_index/src/bit_set.rs:10:42
   |
10 | use rustc_macros::{Decodable, Encodable, HashStableEq};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] rustc_index test:false 0.396
error: could not compile `rustc_index` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] chalk_ir test:false 2.486
