plain
.................................................................................................... 1100/1181
...........iiiiii................................................................
failures:

---- src/os/fd.rs - os::fd::AsFd::as_fd (line 156) stdout ----
error[E0433]: failed to resolve: could not find `wasi` in `os`
 --> src/os/fd.rs:159:14
  |
6 | use std::os::wasi::io::{AsFd, BorrowedFd};
  |              ^^^^ could not find `wasi` in `os`

error[E0412]: cannot find type `BorrowedFd` in this scope
 --> src/os/fd.rs:162:18
  |
9 | let borrowed_fd: BorrowedFd<'_> = f.as_fd();
  |
help: consider importing this struct
  |
3 | use std::os::unix::prelude::BorrowedFd;
3 | use std::os::unix::prelude::BorrowedFd;
  |

error[E0599]: no method named `as_fd` found for struct `File` in the current scope
   --> src/os/fd.rs:162:37
    |
9   | let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    |                                     ^^^^^ method not found in `File`
   ::: /checkout/library/std/src/os/fd.rs:166:8
    |
    |
166 |     fn as_fd(&self) -> BorrowedFd<'_>;
    |        ----- the method is available for `File` here
    = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    |
3   | use std::os::unix::io::AsFd;
---
For more information about an error, try `rustc --explain E0412`.
Couldn't compile the test.

failures:
    src/os/fd.rs - os::fd::AsFd::as_fd (line 156)
test result: FAILED. 1159 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 18.76s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


Build completed unsuccessfully in 0:21:16
