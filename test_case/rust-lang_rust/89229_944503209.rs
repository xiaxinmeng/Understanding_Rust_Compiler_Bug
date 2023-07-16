plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1026 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0785 (line 16482) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0482 (line 8679) stdout ----
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8682:6
  |
3 | fn prefix<'a>(
  |           -- hidden type `Map<impl Iterator<Item = &'a str>, [closure@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6:15: 6:39]>` captures the lifetime `'a` as defined here
4 |     words: impl Iterator<Item = &'a str>
5 | ) -> impl Iterator<Item = String> { // error!
  |
  |
help: to declare that the `impl Trait` captures 'a, you can add an explicit `'a` lifetime bound
  |
5 | ) -> impl Iterator<Item = String> + 'a { // error!

error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
For more information about this error, try `rustc --explain E0700`.
Some expected error codes were not found: ["E0482"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0482 (line 8718) stdout ----
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8721:6
4 |     x: &mut Vec<i32>
4 |     x: &mut Vec<i32>
  |        ------------- hidden type `[closure@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6:5: 9:6]` captures the anonymous lifetime defined here
5 | ) -> impl FnMut(&mut Vec<i32>) -> &[i32] { // error!
  |
  |
help: to declare that the `impl Trait` captures '_, you can add an explicit `'_` lifetime bound
  |
5 | ) -> impl FnMut(&mut Vec<i32>) -> &[i32] + '_ { // error!


error[E0597]: `x` does not live long enough
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8723:18
6  |     |y| {
6  |     |y| {
   |      - has type `&'0 mut Vec<i32>`
7  |         y.append(x);
   |                  ^ `x` would have to be valid for `'0`...
10 | }
   |  -
   |  |
   |  |
   |  ...but `x` will be dropped here, when the function `foo` returns
   |  borrow later used here
   |
   = note: functions cannot return a borrow to data owned within the function's scope, functions can only return borrows to data passed as arguments
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references>
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0700.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
Some expected error codes were not found: ["E0482"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0482 (line 8679)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0482 (line 8718)

