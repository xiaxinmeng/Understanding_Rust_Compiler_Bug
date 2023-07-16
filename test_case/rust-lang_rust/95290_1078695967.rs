plain
......................ii............................................................................ 500/1237
....i....i................................................................i......................ii. 600/1237
.................................................................................................... 700/1237
.................................................................................................... 800/1237
.........F.F........................................................................................ 900/1237
i................................................................................................... 1100/1237
....................................................iiiiii.......................................... 1200/1237
.....i...............................
failures:
---
   |         ^^^^^^^^
   = note: `#[deny(unstable_name_collisions)]` implied by `#[deny(warnings)]`
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `as_bytes(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsStr::as_bytes`
error: aborting due to previous error

Couldn't compile the test.
---- src/os/unix/ffi/mod.rs - os::unix::ffi (line 5) stdout ----
---
   |         ^^^^^^^^
   = note: `#[deny(unstable_name_collisions)]` implied by `#[deny(warnings)]`
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `into_vec(...)` to keep using the current method
   = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable `OsString::into_vec`
error: aborting due to previous error

Couldn't compile the test.

