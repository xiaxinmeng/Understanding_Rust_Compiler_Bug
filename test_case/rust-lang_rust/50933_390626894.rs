plain
[01:27:55] 
[01:27:55] failures:
[01:27:55] 
[01:27:55] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992) stdout ----
[01:27:55] error[E0599]: no method named `recip` found for type `{float}` in the current scope
[01:27:55]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10993:5
[01:27:55]   |
[01:27:55] 3 | 2.0.recip();
[01:27:55] 
[01:27:55] 
[01:27:55] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992)' panicked at 'Some expected error codes were not found: ["E0689"]', librustdoc/test.rs:330:13
[01:27:55] 
[01:27:55] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998) stdout ----
[01:27:55] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998) stdout ----
[01:27:55] error[E0599]: no method named `recip` found for type `{float}` in the current scope
[01:27:55]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11000:3
[01:27:55]   |
[01:27:55] 4 | x.recip();  // same error as above
[01:27:55]   |   ^^^^^
[01:27:55] 
[01:27:55] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998)' panicked at 'Some expected error codes were not found: ["E0689"]', librustdoc/test.rs:330:13
[01:27:55] 
[01:27:55] failures:
[01:27:55]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992)
[01:27:55]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998)
---
[01:27:55] 
[01:27:55] 
[01:27:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:55] Build completed unsuccessfully in 0:42:25
[01:27:55] Makefile:58: recipe for target 'check' failed
[01:27:55] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12ceaada
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
