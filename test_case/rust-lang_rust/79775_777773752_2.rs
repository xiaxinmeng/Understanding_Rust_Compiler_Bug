
running 1 test
test foo.rs - foo (line 1) ... FAILED

failures:

---- foo.rs - foo (line 1) stdout ----
error[E0463]: can't find crate for `foo`
 --> foo.rs:0:1
  |
2 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^ can't find crate
