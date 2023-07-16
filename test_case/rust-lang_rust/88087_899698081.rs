plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1016 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0783 (line 16281) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0161 (line 2806) stdout ----
error[E0277]: the size for values of type `[isize]` cannot be known at compilation time
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2811:32
  |
6 |     let _x: Box<[isize]> = box *array;
  |
  |
  = help: the trait `Sized` is not implemented for `[isize]`
  = note: the type of a box expression must have a statically known size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
Some expected error codes were not found: ["E0161"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0161 (line 2806)

test result: FAILED. 993 passed; 1 failed; 22 ignored; 0 measured; 0 filtered out; finished in 8.62s
