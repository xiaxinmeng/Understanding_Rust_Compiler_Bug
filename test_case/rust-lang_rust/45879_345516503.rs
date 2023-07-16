
[01:06:03] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0644 (line 11807) stdout ----
[01:06:03] 	error[E0631]: type mismatch in closure arguments
[01:06:03]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:12:3
[01:06:03]    |
[01:06:03] 9  |     let x = |y| {
[01:06:03]    |  ___________-
[01:06:03] 10 | |     // Here, when `x` is called, the parameter `y` is equal to `x`.
[01:06:03] 11 | |   };
[01:06:03]    | |___- found signature of `fn(_) -> _`
[01:06:03] 12 |     fix(&x);
[01:06:03]    |     ^^^ expected signature of `for<'r> fn(&'r [closure@/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:9:11: 11:4]) -> _`
[01:06:03]    |
[01:06:03]    = note: required by `fix`
[01:06:03] 
[01:06:03] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:288:12
[01:06:03]
[01:06:03] 
[01:06:03] failures:
[01:06:03]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0644 (line 11807)
