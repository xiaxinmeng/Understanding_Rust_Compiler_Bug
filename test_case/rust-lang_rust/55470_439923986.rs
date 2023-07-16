bash
failures:

---- boxed.rs - boxed::Box<[T]>::from (line 480) stdout ----
error[E0283]: type annotations required: cannot resolve `std::boxed::Box<_>: std::convert::From<&[u8]>`
--> boxed.rs:483:19
|
6 | let boxed_slice = Box::from(slice);
|                   ^^^^^^^^^
|
= note: required by `std::convert::From::from`

thread 'boxed.rs - boxed::Box<[T]>::from (line 480)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
boxed.rs - boxed::Box<[T]>::from (line 480)

test result: FAILED. 408 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--doc'
