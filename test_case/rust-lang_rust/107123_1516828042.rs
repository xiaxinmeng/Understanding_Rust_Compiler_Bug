plain
doc tests for: /checkout/src/doc/unstable-book/src/library-features/default-free-fn.md
doc tests for: /checkout/src/doc/unstable-book/src/library-features/fn-traits.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/library-features/fn-traits.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/library-features/fn-traits.md - The_tracking_issue_for_this_feature_is_ (line 16) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/library-features/fn-traits.md - The_tracking_issue_for_this_feature_is_ (line 16) stdout ----
error[E0277]: the trait bound `Adder: Callable<(u32,)>` is not satisfied
    |
    |
9   | impl FnOnce<(u32, )> for Adder {
    |                          ^^^^^ the trait `Callable<(u32,)>` is not implemented for `Adder`
note: required by a bound in `FnOnce`
   --> /checkout/library/core/src/ops/function.rs:243:32
    |
    |
243 | pub trait FnOnce<Args: Tuple>: ~const Callable<Args> {
    |                                ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `FnOnce`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
Couldn't compile the test.
