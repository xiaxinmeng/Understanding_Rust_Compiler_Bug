
[01:38:35] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0659 (line 11886) stdout ----
[01:38:35] 	error[E0365]: `moon` is private, and cannot be reexported
[01:38:35]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11896:13
[01:38:35]    |
[01:38:35] 11 |     pub use moon;
[01:38:35]    |             ^^^^ reexport of private `moon`
[01:38:35]    |
[01:38:35]    = note: consider declaring type or module `moon` with `pub`
[01:38:35] 
[01:38:35] error[E0365]: `earth` is private, and cannot be reexported
[01:38:35]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11897:13
[01:38:35]    |
[01:38:35] 12 |     pub use earth;
[01:38:35]    |             ^^^^^ reexport of private `earth`
[01:38:35]    |
[01:38:35]    = note: consider declaring type or module `earth` with `pub`
[01:38:35] 
[01:38:35] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:289:13
