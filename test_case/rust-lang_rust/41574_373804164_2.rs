console
$ rustdoc +nightly --version
rustdoc 1.26.0-nightly (392645394 2018-03-15)
$ rustc +nightly g.rs
warning: struct is never used: `OtherStruct`
  --> g.rs:12:1
   |
12 | struct OtherStruct;
   | ^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(dead_code)] on by default

$ rustdoc +nightly g.rs
(no output)
$ rustdoc +nightly g.rs -Z unstable-options --display-warnings
(no output)
$ rustdoc +nightly --test g.rs

running 1 test
test g.rs - SomeStruct (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ rustdoc +nightly --test g.rs -Z unstable-options --display-warnings

running 1 test
test g.rs - SomeStruct (line 6) ... ok

successes:

---- g.rs - SomeStruct (line 6) stdout ----
        warning: unused variable: `x`
 --> g.rs:7:5
  |
3 | let x = 5;
  |     ^ help: consider using `_x` instead
  |
  = note: #[warn(unused_variables)] on by default



successes:
    g.rs - SomeStruct (line 6)

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ rustdoc +nightly --test g.rs --test-args "--nocapture"

running 1 test
warning: unused variable: `x`
 --> g.rs:7:5
  |
3 | let x = 5;
  |     ^ help: consider using `_x` instead
  |
  = note: #[warn(unused_variables)] on by default

test g.rs - SomeStruct (line 6) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
