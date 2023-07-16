plain
[01:53:37] 
[01:53:37] failures:
[01:53:37] 
[01:53:37] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992) stdout ----
[01:53:37] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992)' panicked at 'test compiled while it wasn't supposed to', librustdoc/test.rs:315:17
[01:53:37] 
[01:53:37] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998) stdout ----
[01:53:37] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998) stdout ----
[01:53:37] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998)' panicked at 'test compiled while it wasn't supposed to', librustdoc/test.rs:315:17
[01:53:37] 
[01:53:37] failures:
[01:53:37]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10992)
[01:53:37]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0689 (line 10998)
---
[01:53:37] 
[01:53:37] 
[01:53:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:53:37] Build completed unsuccessfully in 0:56:07
[01:53:37] make: *** [check] Error 1
[01:53:37] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06a72080
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
