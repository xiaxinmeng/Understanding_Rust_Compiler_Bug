
[01:05:30] failures:
[01:05:30] 
[01:05:30] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 4928) stdout ----
[01:05:30] 	error[E0631]: type mismatch in closure arguments
[01:05:30]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7:5
[01:05:30]   |
[01:05:30] 7 |     foo(|y: String| { });
[01:05:30]   |     ^^^ --------------- takes argument of type `std::string::String`
[01:05:30]   |     |
[01:05:30]   |     expected argument of type `usize`
[01:05:30]   |
[01:05:30]   = note: required by `foo`
[01:05:30] 
[01:05:30] thread 'rustc' panicked at 'Some expected error codes were not found: ["E0281"]', /checkout/src/librustdoc/test.rs:286:8
[01:05:30] 
[01:05:30] 
[01:05:30] failures:
[01:05:30]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 4928)
[01:05:30] 
[01:05:30] test result: FAILED. 659 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out
[01:05:30] 
[01:05:30] 
