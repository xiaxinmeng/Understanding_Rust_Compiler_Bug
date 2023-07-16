
$ rustc -o test test.rs
test.rs:1:1: 1:21 error: can't find crate for `submod`
test.rs:1 extern crate submod;
          ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
