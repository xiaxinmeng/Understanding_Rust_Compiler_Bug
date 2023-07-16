plain
[00:52:09] ...................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:52:32] .................................................................................
[00:52:51] ....................................................................................ii..............
[00:53:42] ................................................i...................................................
[00:53:53] .i.ii...........................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:54:42] .........iiiiiii....................................................................................
[00:55:03] ....................................................................................................
[00:55:21] ....................................................................................................
[00:55:40] ...................................................................................
---
[01:25:19] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0016 (line 338) stdout ----
[01:25:19]  error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:25:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:339:28
[01:25:19]   |
[01:25:19] 3 | const FOO: i32 = { let x = 0; x }; // 'x' isn't an item!
[01:25:19]   |
[01:25:19]   |
[01:25:19]   = help: add #![feature(const_let)] to the crate attributes to enable
[01:25:19] error[E0658]: statements in constants are unstable (see issue #48821)
[01:25:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:339:28
[01:25:19]   |
[01:25:19]   |
[01:25:19] 3 | const FOO: i32 = { let x = 0; x }; // 'x' isn't an item!
[01:25:19]   |
[01:25:19]   |
[01:25:19]   = help: add #![feature(const_let)] to the crate attributes to enable
[01:25:19] error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:25:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:339:31
[01:25:19]   |
[01:25:19]   |
[01:25:19] 3 | const FOO: i32 = { let x = 0; x }; // 'x' isn't an item!
[01:25:19]   |
[01:25:19]   |
[01:25:19]   = help: add #![feature(const_let)] to the crate attributes to enable
[01:25:19] error[E0658]: let bindings in constants are unstable (see issue #48821)
[01:25:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:339:1
[01:25:19]   |
[01:25:19]   |
[01:25:19] 3 | const FOO: i32 = { let x = 0; x }; // 'x' isn't an item!
[01:25:19]   |
[01:25:19]   |
[01:25:19]   = help: add #![feature(const_let)] to the crate attributes to enable
[01:25:19] error[E0658]: statements in constants are unstable (see issue #48821)
[01:25:19]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:339:1
[01:25:19]   |
[01:25:19]   |
[01:25:19] 3 | const FOO: i32 = { let x = 0; x }; // 'x' isn't an item!
[01:25:19]   |
[01:25:19]   |
[01:25:19]   = help: add #![feature(const_let)] to the crate attributes to enable
[01:25:19] 
[01:25:19] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0016 (line 338)' panicked at 'Some expected error codes were not found: ["E0016"]', librustdoc/test.rs:326:9
[01:25:19] 
[01:25:19] 
[01:25:19] failures:
[01:25:19]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0016 (line 338)
---
[01:25:19] 
[01:25:19] 
[01:25:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:19] Build completed unsuccessfully in 0:43:07
[01:25:19] Makefile:58: recipe for target 'check' failed
[01:25:19] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:045edd34
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
