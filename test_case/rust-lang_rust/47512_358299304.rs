
[01:28:44] failures:
[01:28:44] 
[01:28:44] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0659 (line 11858) stdout ----
[01:28:44] 	error[E0432]: unresolved import `moon`
[01:28:44]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11868:13
[01:28:44]    |
[01:28:44] 12 |     pub use moon::*;
[01:28:44]    |             ^^^^ Maybe a missing `extern crate moon;`?
[01:28:44] 
[01:28:44] error[E0432]: unresolved import `earth`
[01:28:44]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11869:13
[01:28:44]    |
[01:28:44] 13 |     pub use earth::*;
[01:28:44]    |             ^^^^^ Maybe a missing `extern crate earth;`?
[01:28:44] 
[01:28:44] error[E0425]: cannot find function `foo` in module `collider`
[01:28:44]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11872:11
[01:28:44]    |
[01:28:44] 16 | collider::foo(); // ERROR: `foo` is ambiguous
[01:28:44]    |           ^^^ not found in `collider`
[01:28:44] 
[01:28:44] thread 'rustc' panicked at 'Some expected error codes were not found: ["E0659"]', librustdoc/test.rs:294:9
[01:28:44] 
[01:28:44] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0659 (line 11884) stdout ----
[01:28:44] 	error[E0432]: unresolved import `moon`
[01:28:44]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11894:13
[01:28:44]    |
[01:28:44] 12 |     pub use moon;
[01:28:44]    |             ^^^^ no `moon` in the root
[01:28:44] 
[01:28:44] error[E0432]: unresolved import `earth`
[01:28:44]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11895:13
[01:28:44]    |
[01:28:44] 13 |     pub use earth;
[01:28:44]    |             ^^^^^ no `earth` in the root
[01:28:44] 
[01:28:44] thread 'rustc' panicked at 'couldn't compile the test', librustdoc/test.rs:289:13
[01:28:44] 
[01:28:44] 
[01:28:44] failures:
[01:28:44]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0659 (line 11858)
[01:28:44]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0659 (line 11884)
[01:28:44] 
[01:28:44] test result: FAILED. 662 passed; 2 failed; 22 ignored; 0 measured; 0 filtered out
