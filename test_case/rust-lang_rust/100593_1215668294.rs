plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: passing `TypeError<'_>` by reference
  --> compiler/rustc_middle/src/ty/error.rs:78:29
   |
78 |     pub fn involves_regions(&self) -> bool {
   |                             ^^^^^ help: try passing by value: `TypeError<'_>`
   |
   = note: `-D rustc::pass-by-value` implied by `-D warnings`
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
Build completed unsuccessfully in 0:02:05
