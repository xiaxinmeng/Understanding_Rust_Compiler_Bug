plain
.................................................................................................... 1100/1177
.......iiiiii................................................................
failures:

---- src/os/./unix/io/fd.rs - os::doc::unix::io::fd::AsFd::as_fd (line 151) stdout ----
error[E0658]: use of unstable library feature 'io_safety'
 --> src/os/./unix/io/fd.rs:157:18
  |
9 | let borrowed_fd: BorrowedFd<'_> = f.as_fd();
  |
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = help: add `#![feature(io_safety)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'io_safety'
 --> src/os/./unix/io/fd.rs:154:25
  |
6 | use std::os::unix::io::{AsFd, BorrowedFd};
  |
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = help: add `#![feature(io_safety)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'io_safety'
 --> src/os/./unix/io/fd.rs:154:31
  |
6 | use std::os::unix::io::{AsFd, BorrowedFd};
  |
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = help: add `#![feature(io_safety)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'io_safety'
 --> src/os/./unix/io/fd.rs:157:37
  |
9 | let borrowed_fd: BorrowedFd<'_> = f.as_fd();
  |
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = note: see issue #87074 <https://github.com/rust-lang/rust/issues/87074> for more information
  = help: add `#![feature(io_safety)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    src/os/./unix/io/fd.rs - os::doc::unix::io::fd::AsFd::as_fd (line 151)
test result: FAILED. 1155 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 14.76s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:16:20
