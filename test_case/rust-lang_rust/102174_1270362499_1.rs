
compiling  ptr::c::rustc_calls_cc
running: "rustc" "--crate-type" "staticlib" "--out-dir" "target/temp/" "--target" "i686-unknown-linux-gnu" "-Cmetadata=ptr_c_rustc_caller" "generated_impls/rustc/ptr_c_rustc_caller.rs"
Failed to build test: rust compile error 
 
warning: associated function `new` is never used
  --> generated_impls/rustc/ptr_c_rustc_caller.rs:33:8
   |
33 |     fn new(val: i128) -> Self {
   |        ^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: associated function `new` is never used
  --> generated_impls/rustc/ptr_c_rustc_caller.rs:42:8
   |
42 |     fn new(val: u128) -> Self {
   |        ^^^

error: literal out of range for `usize`
   --> generated_impls/rustc/ptr_c_rustc_caller.rs:718:29
    |
718 |         let arg0: *mut () = 0x706050403020100 as *mut ();
    |                             ^^^^^^^^^^^^^^^^^
    |
    = note: the literal `0x706050403020100` (decimal `506097522914230528`) does not fit into the type `usize` and will become `50462976usize`
    = note: `#[deny(overflowing_literals)]` on by default

[...]
