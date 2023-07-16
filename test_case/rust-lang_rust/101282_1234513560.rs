plain
---- src\os\fd\owned.rs - os::fd::owned::AsFd::as_fd (line 205) stdout ----
error[E0432]: unresolved import `std::os::fd`
 --> src\os\fd\owned.rs:208:14
  |
6 | use std::os::fd::{AsFd, BorrowedFd};
  |              ^^ could not find `fd` in `os`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src\os\fd\raw.rs - os::fd::raw::IntoRawFd::into_raw_fd (line 115) stdout ----
error[E0432]: unresolved import `std::os::fd`
 --> src\os\fd\raw.rs:118:14
  |
6 | use std::os::fd::{IntoRawFd, RawFd};
  |              ^^ could not find `fd` in `os`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src\os\fd\raw.rs - os::fd::raw::FromRawFd::from_raw_fd (line 80) stdout ----
error[E0432]: unresolved import `std::os::fd`
 --> src\os\fd\raw.rs:83:14
  |
6 | use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
  |              ^^ could not find `fd` in `os`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
Couldn't compile the test.
---- src\os\fd\raw.rs - os::fd::raw::AsRawFd::as_raw_fd (line 42) stdout ----
error[E0432]: unresolved import `std::os::fd`
 --> src\os\fd\raw.rs:45:14
  |
6 | use std::os::fd::{AsRawFd, RawFd};
  |              ^^ could not find `fd` in `os`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.
---
test result: FAILED. 1074 passed; 4 failed; 18 ignored; 0 measured; 0 filtered out; finished in 35.16s

error: test failed, to rerun pass '--doc'
Build completed unsuccessfully in 0:38:46
make: *** [Makefile:73: ci-subset-1] Error 1
