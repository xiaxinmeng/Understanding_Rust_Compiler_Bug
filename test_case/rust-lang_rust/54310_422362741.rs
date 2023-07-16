plain
[00:56:24] ....................................................................................................
[00:56:27] .....................................................i..............................................
[00:56:30] ....................................................................................................
[00:56:33] ....................................................................................................
[00:56:36] .iiiiiiiii..........................................................................................
[00:56:42] ....................................................................................................
[00:56:45] ..................................................................................i.................
[00:56:48] ....................................................................................................
[00:56:51] ....................................i.i..ii.........................................................
---
[01:30:47] 
[01:30:47] failures:
[01:30:47] 
[01:30:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11417) stdout ----
[01:30:47] error[E0621]: explicit lifetime required in the type of `s`
[01:30:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11424:73
[01:30:47]   |
[01:30:47] 9 | fn demo<'a>(s: &mut S<'a>) -> &'a mut String { let p = &mut *(*s).data; p }
[01:30:47]   |                ----------                                               ^ lifetime `'a` required
[01:30:47]   |                |
[01:30:47]   |                help: add explicit lifetime `'a` to the type of `s`: `&'a mut main::S<'a>`
[01:30:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11417)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:30:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:30:47] 
[01:30:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11389) stdout ----
[01:30:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11389) stdout ----
[01:30:47] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0713 (line 11389)' panicked at 'test compiled while it wasn't supposed to', librustdoc/test.rs:323:13
[01:30:47] 
[01:30:47] failures:
[01:30:47]     /checkout//rustlib
74124 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
---
travis_time:end:1e152650:start=1537271547467579291,finish=1537271547473511863,duration=5932572
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d76b9d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; the
