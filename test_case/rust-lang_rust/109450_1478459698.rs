plain
   Compiling core v0.0.0 (/checkout/library/core)
error: unknown lint: `fuzzy_provenance_casts`
  --> library/alloc/benches/lib.rs:10:1
   |
10 | #![deny(fuzzy_provenance_casts)]
   |
   |
   = note: the `fuzzy_provenance_casts` lint is unstable
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
   = help: add `#![feature(strict_provenance)]` to the crate attributes to enable
   = note: `-D unknown-lints` implied by `-D warnings`
[RUSTC-TIMING] collectionsbenches test:true 0.029
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] run_time_detect test:true 0.109
