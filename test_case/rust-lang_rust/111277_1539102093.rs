plain
---- src/io/mod.rs - io::IoSlice<'a>::into_bytes (line 1358) stdout ----
error[E0658]: use of unstable library feature 'io_slice_as_bytes'
 --> src/io/mod.rs:1364:21
  |
9 | let tail = io_slice.into_bytes()[3..];
  |
  = note: see issue #111277 <https://github.com/rust-lang/rust/issues/111277> for more information
  = help: add `#![feature(io_slice_as_bytes)]` to the crate attributes to enable


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> src/io/mod.rs:1364:5
  |
9 | let tail = io_slice.into_bytes()[3..];
  |
  = help: the trait `Sized` is not implemented for `[u8]`
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature
  = help: unsized locals are gated as an unstable feature
help: consider borrowing here
  |
9 | let tail = &io_slice.into_bytes()[3..];

error[E0308]: mismatched types
    --> src/io/mod.rs:1367:25
     |
     |
12   | io_slice = IoSlice::new(tail);
     |            |            |
     |            |            |
     |            |            expected `&[u8]`, found `[u8]`
     |            |            help: consider borrowing here: `&tail`
     |
note: associated function defined here
    --> /checkout/library/std/src/io/mod.rs:1262:12
     |
     |
1262 |     pub fn new(buf: &'a [u8]) -> IoSlice<'a> {

error[E0658]: use of unstable library feature 'io_slice_as_bytes'
  --> src/io/mod.rs:1369:21
   |
   |
14 | assert_eq!(io_slice.into_bytes(), b"def");
   |
   = note: see issue #111277 <https://github.com/rust-lang/rust/issues/111277> for more information
   = help: add `#![feature(io_slice_as_bytes)]` to the crate attributes to enable

---
    src/io/mod.rs - io::IoSlice<'a>::into_bytes (line 1358)

test result: FAILED. 1116 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 15.41s

error: doctest failed, to rerun pass `-p std --doc`
