plain
   Compiling snap v1.0.1
error: an associated function with this name may be added to the standard library in the future
  --> compiler/rustc_fs_util/src/lib.rs:85:20
   |
85 |     CString::new(p.as_bytes()).unwrap()
   |
   |
   = note: `-D unstable-name-collisions` implied by `-D warnings`
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `as_bytes(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsStr::as_bytes`
    Checking ansi_term v0.12.1
    Checking unicode-script v0.5.3
error: could not compile `rustc_fs_util` due to previous error
warning: build failed, waiting for other jobs to finish...
