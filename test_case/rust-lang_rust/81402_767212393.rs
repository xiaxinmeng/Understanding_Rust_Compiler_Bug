plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.066 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i.....ii.........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.23s

 finished in 2.305 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/intrinsics.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/lang-items.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/language-features/lang-items.md" "--test-args" ""

stdout ----

running 3 tests
running 3 tests
test /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_::_::Writing_an_executable_without_stdlib (line 144) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_ (line 18) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_::_::Writing_an_executable_without_stdlib (line 108) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_::_::Writing_an_executable_without_stdlib (line 144) stdout ----
error[E0464]: multiple matching crates for `libc`
   |
10 | extern crate libc;
   | ^^^^^^^^^^^^^^^^^^
   |
   |
   = note: candidates:
           crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-d630318955afa9cb.rlib
           crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-c4a7f3f5739debac.rlib
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_ (line 18) stdout ----
---- /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_ (line 18) stdout ----
error[E0464]: multiple matching crates for `libc`
  |
7 | extern crate libc;
  | ^^^^^^^^^^^^^^^^^^
  |
  |
  = note: candidates:
          crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-d630318955afa9cb.rlib
          crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-c4a7f3f5739debac.rlib
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_::_::Writing_an_executable_without_stdlib (line 108) stdout ----
---- /checkout/src/doc/unstable-book/src/language-features/lang-items.md - The_tracking_issue_for_this_feature_is__None_::_::Writing_an_executable_without_stdlib (line 108) stdout ----
error[E0464]: multiple matching crates for `libc`
  |
9 | extern crate libc;
  | ^^^^^^^^^^^^^^^^^^
  |
  |
  = note: candidates:
          crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-d630318955afa9cb.rlib
          crate `libc`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-c4a7f3f5739debac.rlib
error: aborting due to previous error

Couldn't compile the test.

