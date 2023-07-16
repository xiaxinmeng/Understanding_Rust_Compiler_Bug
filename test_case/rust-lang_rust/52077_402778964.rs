plain
[01:23:24] 
[01:23:24] failures:
[01:23:24] 
[01:23:24] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0705 (line 11346) stdout ----
[01:23:24] error[E0705]: types cannot be named `dyn`
[01:23:24]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11347:1
[01:23:24]   |
[01:23:24] 3 | trait dyn { }
[01:23:24] 
[01:23:24] 
[01:23:24] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0705 (line 11346)' panicked at 'Some expected error codes were not found: ["E0695"]', librustdoc/test.rs:332:13
[01:23:24] 
[01:23:24] 
[01:23:24] failures:
[01:23:24]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0705 (line 11346)
---
[01:23:24] 
[01:23:24] 
[01:23:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:24] Build completed unsuccessfully in 0:42:49
[01:23:24] Makefile:58: recipe for target 'check' failed
[01:23:24] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03796101
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
