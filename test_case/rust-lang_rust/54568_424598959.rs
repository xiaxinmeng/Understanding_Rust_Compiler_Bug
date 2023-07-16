plain
[00:58:54] ....................................................................................................
[00:58:57] ................................................................i...................................
[00:59:00] ....................................................................................................
[00:59:03] ....................................................................................................
[00:59:06] .............iiiiiiiii..............................................................................
[00:59:12] ....................................................................................................
[00:59:16] ................................................................................................i...
[00:59:18] ....................................................................................................
[00:59:21] ........................................................i.i..ii.....................................
---
[01:36:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0668 (line 11046) stdout ----
[01:36:02] error[E0668]: malformed inline assembly
[01:36:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11052:9
[01:36:02]   |
[01:36:02] 7 |         asm!("" :"={rax"(rax));
[01:36:02] 
[01:36:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0668 (line 11046)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:36:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:36:02] 
---
[01:36:02] 
[01:36:02] 
[01:36:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:02] Build completed unsuccessfully in 0:46:16
[01:36:02] make: *** [check] Error 1
[01:36:02] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c2dd0b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
