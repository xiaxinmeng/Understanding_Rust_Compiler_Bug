
> rustdoc +rustc2 --test doctest.rs 

running 1 test
test doctest.rs - (line 1) ... FAILED

failures:

---- doctest.rs - (line 1) stdout ----
error: expected one of `;` or `as`, found `<eof>`
 --> doctest.rs:2:14
  |
2 | extern crate p
  |              ^ expected one of `;` or `as`
