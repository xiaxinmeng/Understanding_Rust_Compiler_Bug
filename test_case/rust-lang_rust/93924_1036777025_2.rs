
$ cargo +nightly test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests (target/debug/deps/impl_type_coercion-5c03dc1e98d7ad90)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests impl-type-coercion

running 1 test
test src/lib.rs - maybe_source (line 16) ... FAILED

failures:

---- src/lib.rs - maybe_source (line 16) stdout ----
error[E0308]: mismatched types
 --> src/lib.rs:19:1
  |
3 | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_lib_rs_16_0() -> Result<(), impl core::fmt::Debug> {
  |                                                                                       --------------------- the expected opaque type
...
6 | Ok::<_, Target>(())
  | ^^^^^^^^^^^^^^^^^^^ expected struct `Source`, found struct `Target`
  |
  = note: expected opaque type `impl Debug`
                  found struct `Target`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
Couldn't compile the test.

failures:
    src/lib.rs - maybe_source (line 16)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass '--doc'
$
