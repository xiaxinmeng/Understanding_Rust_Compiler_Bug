plain
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
error: unused import: `HashStableEq`
  --> compiler/rustc_index/src/bit_set.rs:10:42
   |
10 | use rustc_macros::{Decodable, Encodable, HashStableEq};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_index` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:50
