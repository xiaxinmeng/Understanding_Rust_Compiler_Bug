plain
[00:02:53]    Compiling cc v1.0.10
[00:02:53]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:53]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:02:53]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:54] error: expected expression, found `u8`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4560 | impl_from_itself!(u8, #[stable(since = "1.27.0")]);
[00:02:54]      | --------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `u16`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4561 | impl_from_itself!(u16, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `u32`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4562 | impl_from_itself!(u32, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `u64`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4563 | impl_from_itself!(u64, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `u128`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4564 | impl_from_itself!(u128, #[stable(since = "1.27.0")]);
[00:02:54]      | ----------------------------------------------------- in this macro invocation
[00:02:54] error: expected expression, found `i8`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54]      |
[00:02:54] 4554 |                 $nb
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] 4566 | impl_from_itself!(i8, #[stable(since = "1.27.0")]);
[00:02:54]      | --------------------------------------------------- in this macro invocation
[00:02:54] error: expected expression, found `i16`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54]      |
[00:02:54] 4554 |                 $nb
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] 4567 | impl_from_itself!(i16, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] error: expected expression, found `i32`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54]      |
[00:02:54] 4554 |                 $nb
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] 4568 | impl_from_itself!(i32, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] error: expected expression, found `i64`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54]      |
[00:02:54] 4554 |                 $nb
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] 4569 | impl_from_itself!(i64, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] error: expected expression, found `i128`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54]      |
[00:02:54] 4554 |                 $nb
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] 4570 | impl_from_itself!(i128, #[stable(since = "1.27.0")]);
[00:02:54]      | ----------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `f32`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4572 | impl_from_itself!(f32, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:54] 
[00:02:54] error: expected expression, found `f64`
[00:02:54]     --> libcore/num/mod.rs:4554:17
[00:02:54] 4554 |                 $nb
[00:02:54]      |                 ^^^ expected expression
[00:02:54] ...
[00:02:54] ...
[00:02:54] 4573 | impl_from_itself!(f64, #[stable(since = "1.27.0")]);
[00:02:54]      | ---------------------------------------------------- in this macro invocation
[00:02:55] error: aborting due to 12 previous errors
[00:02:55] 
[00:02:55] error: Could not compile `core`.
[00:02:55] 
[00:02:55] 
[00:02:55] Caused by:
[00:02:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=1b8789e893adb899 -C extra-filename=-1b8789e893adb899 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:02:55] warning: build failed, waiting for other jobs to finish...
[00:03:00] error: build failed
[00:03:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:00] expected success, got: exit code: 101
[00:03:00] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:00] travis_fold:end:stage0-std

[00:03:00] travis_time:end:stage0-std:start=1524560396048294343,finish=1524560402947693983,duration=6899399640


[00:03:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:00] Build completed unsuccessfully in 0:00:08
[00:03:00] Makefile:79: recipe for target 'tidy' failed
[00:03:00] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1b210103
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
