plain
....................................................iiiiii.......................................... 1200/1237
.....i...............................
failures:

---- src/ffi/os_str.rs - ffi::os_str::OsStr::as_bytes (line 706) stdout ----
error[E0658]: use of unstable library feature 'osstr_bytes'
  |
  |
7 | assert_eq!(os_str.as_bytes(), b"foo");
  |
  = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
---- src/ffi/os_str.rs - ffi::os_str::OsString::into_vec (line 185) stdout ----
error[E0658]: use of unstable library feature 'osstr_bytes'
  |
  |
7 | let v = os_string.into_vec();
  |
  = help: add `#![feature(osstr_bytes)]` to the crate attributes to enable

error: aborting due to previous error
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

