plain
---- [rustdoc] rustdoc/process-termination.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "--deny" "warnings" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
------------------------------------------

running 3 tests
test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 7) ... ok
test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 14) ... ok

failures:

---- /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) stdout ----
error[E0277]: `?` couldn't convert the error to `impl Debug`
  |
  |
2 | ...n _doctest_main__checkout_src_test_rustdoc_process_termination_rs_20_0() -> Result<(), impl core::fmt::Debug> {
  |                                                                                --------------------------------- expected `impl Debug` because of this
3 | ...ding to panic")?;
  |                   ^ the trait `From<&str>` is not implemented for `impl Debug`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
  = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, &str>>` for `Result<(), impl Debug>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
