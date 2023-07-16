plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 109 tests
ii..i..i.....i......i.ii.i.......i......iii.....................i...........iF.i..............i..... 100/109
failures:

---- src/builtin.rs - builtin::UNSAFE_IMPL_DEPRECATED_SAFE (line 3883) stdout ----
error: expected item, found `;`
error: expected item, found `;`
 --> src/builtin.rs:3891:20
  |
9 | impl Foo for Bar {};
  |                    ^ help: remove this semicolon

error[E0200]: the trait `Foo` requires an `unsafe impl` declaration
  |
9 | impl Foo for Bar {};
  | ^^^^^^^^^^^^^^^^^^^

