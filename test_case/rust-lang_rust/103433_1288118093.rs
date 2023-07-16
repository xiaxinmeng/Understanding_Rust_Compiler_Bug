plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1047 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0313 (line 5591) stdout ----
error: lifetime may not live long enough
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5593:8
  |
4 |     || foo(bar);
  |     -- ^^^^^^^^ argument requires that `'1` must outlive `'static`
  |     |
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `bar` does not live long enough
  |
  |
4 |     || foo(bar);
  |     -- ----^^^-
  |     |  |   |
  |     |  |   borrowed value does not live long enough
  |     |  argument requires that `bar` is borrowed for `'static`
5 | }
5 | }
  |  - `bar` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
For more information about this error, try `rustc --explain E0597`.
Some expected error codes were not found: ["E0313"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0313 (line 5622) stdout ----
error: lifetime may not live long enough
  |
  |
4 |     move || foo(bar);
  |     ------- ^^^^^^^^ argument requires that `'1` must outlive `'static`
  |     |
  |     lifetime `'1` represents this closure's body
  |
  = note: closure implements `FnMut`, so references to captured variables can't escape the closure
error: aborting due to previous error

Couldn't compile the test.

