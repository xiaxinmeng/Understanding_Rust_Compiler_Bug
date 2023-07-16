plain
[01:23:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0538 (line 9384) stdout ----
[01:23:59] error: unexpected token: `,`
[01:23:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:9386:19
[01:23:59]   |
[01:23:59] 4 |     since="1.0.0"),
[01:23:59]   |                   ^ unexpected token after this
[01:23:59] 
[01:23:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0538 (line 9384)' panicked at 'Some expected error codes were not found: ["E0538"]', librustdoc/test.rs:330:13
[01:23:59] 
[01:23:59] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0538 (line 9400) stdout ----
[01:23:59] error: unexpected token: `,`
[01:23:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:9402:19
[01:23:59]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:9402:19
[01:23:59]   |
[01:23:59] 4 |     since="1.0.0"),
[01:23:59]   |                   ^ unexpected token after this
[01:23:59] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0538 (line 9400)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:23:59] 
[01:23:59] 
[01:23:59] failures:
---
[01:23:59] 
[01:23:59] 
[01:23:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:59] Build completed unsuccessfully in 0:41:19
[01:23:59] make: *** [check] Error 1
[01:23:59] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0cb4f3bb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
