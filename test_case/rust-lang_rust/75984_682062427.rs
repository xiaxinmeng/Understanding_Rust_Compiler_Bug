
failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0433 (line 7746) stdout ----
error[E0432]: unresolved import `rand`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7747:5
  |
3 | use rand::Rng;
  |     ^^^^ maybe a missing crate `rand`?

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
Some expected error codes were not found: ["E0433"]

failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0433 (line 7746)
