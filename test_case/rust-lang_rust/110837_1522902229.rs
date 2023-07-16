plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/intra-doc-pointers.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/intrinsics.md


command did not execute successfully: CFG_RELEASE_CHANNEL="nightly" RUSTC_BOOTSTRAP="1" RUSTC_STAGE="2" RUSTC_SYSROOT="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" RUSTDOC_LIBDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" RUSTDOC_REAL="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_TEST_THREADS="16" "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/intrinsics.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/language-features/intrinsics.md - The_tracking_issue_for_this_feature_is__None_ (line 18) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/intrinsics.md - The_tracking_issue_for_this_feature_is__None_ (line 18) stdout ----
error[E0094]: intrinsic has wrong number of type parameters: found 1, expected 2
  |
  |
8 |     fn offset<T>(dst: *const T, offset: isize) -> *const T;
  |              ^^^ expected 2 type parameters
error: aborting due to previous error

For more information about this error, try `rustc --explain E0094`.
Couldn't compile the test.
