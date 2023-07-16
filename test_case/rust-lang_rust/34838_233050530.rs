
run doc-book-no-stdlib [x86_64-unknown-linux-gnu]

running 2 tests

test _0 ... ignored

test _1 ... FAILED

failures:

---- _1 stdout ----

    <anon>:7:1: 7:19 error: use of unstable library feature 'libc': use `libc` from crates.io (see issue #27783)

<anon>:7 extern crate libc;

         ^~~~~~~~~~~~~~~~~~

<anon>:7:1: 7:19 help: add #![feature(libc)] to the crate attributes to enable

error: aborting due to previous error(s)

thread '_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:170

note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:

    _1

test result: FAILED. 0 passed; 1 failed; 1 ignored; 0 measured
