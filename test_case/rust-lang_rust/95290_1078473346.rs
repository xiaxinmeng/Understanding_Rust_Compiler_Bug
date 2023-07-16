plain
   |
42 |         path.as_bytes()
   |              ^^^^^^^^
   |
   = note: `-D unstable-name-collisions` implied by `-D warnings`
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `as_bytes(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsStr::as_bytes`
error: could not compile `rustc_codegen_cranelift` due to previous error
Build completed unsuccessfully in 0:03:01
