plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.069 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i..ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.405 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 982 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0779 (line 15704) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0074::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1559) stdout ----
error[E0075]: SIMD vector length must be a power of two
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1563:1
  |
6 | struct Bad<T>(T, T, T);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
For more information about this error, try `rustc --explain E0075`.
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0074::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1568) stdout ----
error[E0075]: SIMD vector length must be a power of two
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1572:1
  |
6 | struct Good(u32, u32, u32);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
For more information about this error, try `rustc --explain E0075`.
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0076 (line 1619) stdout ----
error[E0075]: SIMD vector length must be a power of two
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1623:1
  |
6 | struct Good(u32, u32, u32); // ok!

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
For more information about this error, try `rustc --explain E0075`.
Couldn't compile the test.
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0077 (line 1644) stdout ----
error[E0075]: SIMD vector length must be a power of two
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1648:1
  |
6 | struct Good(u32, u32, u32); // ok!

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
