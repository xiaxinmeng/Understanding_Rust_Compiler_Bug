plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: unused import: `Rng`
  --> compiler/rustc_middle/src/ty/layout.rs:27:30
   |
27 | use rand::{seq::SliceRandom, Rng, SeedableRng};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:11
