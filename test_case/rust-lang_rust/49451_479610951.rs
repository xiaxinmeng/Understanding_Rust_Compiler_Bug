console
$ rustdoc +stable --test f.rs --extern f=libf.rlib

running 1 test
test f.rs - SomeStruct::foo (line 10) ... FAILED

failures:

---- f.rs - SomeStruct::foo (line 10) stdout ----
error[E0432]: unresolved import `crate::SomeStruct`
 --> f.rs:11:5
  |
4 | use crate::SomeStruct;
  |     ^^^^^^^^^^^^^^^^^ no `SomeStruct` in the root

thread 'f.rs - SomeStruct::foo (line 10)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    f.rs - SomeStruct::foo (line 10)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
