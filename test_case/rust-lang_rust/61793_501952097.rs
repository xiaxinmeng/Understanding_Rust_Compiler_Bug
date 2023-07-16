
   Compiling rustc_macros v0.1.0 (/Users/matthias/Code/rust/rust/src/librustc_macros)
error[E0460]: found possibly newer version of crate `std` which `synstructure` depends on
 --> src/librustc_macros/src/lib.rs:6:5
  |
6 | use synstructure::decl_derive;
  |     ^^^^^^^^^^^^
  |
  = note: perhaps that crate needs to be recompiled?
  = note: the following crate versions were found:
          crate `std`: /Users/matthias/Code/rust/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libstd-5ab454490186c8f5.rlib
          crate `std`: /Users/matthias/Code/rust/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libstd-5ab454490186c8f5.dylib
          crate `synstructure`: /Users/matthias/Code/rust/rust/build/x86_64-apple-darwin/stage1-rustc/release/deps/libsynstructure-7d32b126cebc2b54.rlib

error: aborting due to previous error

error: Could not compile `rustc_macros`.
warning: build failed, waiting for other jobs to finish...
error: build failed
