plain
[00:02:52]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:52]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:02:52]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:55]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:02:55] error[E0425]: cannot find value `u128` in module `self::shift_max`
[00:02:55]    --> libcore/num/wrapping.rs:115:29
[00:02:55]     |
[00:02:55] 115 |         sh_impl_unsigned! { $t, usize }
[00:02:55]     |                             ^^ not found in `self::shift_max`
[00:02:55] ...
[00:02:55] 125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
[00:02:55]     | -------------------------------------------------------------------- in this macro invocation
[00:02:55] 
[00:02:55] error[E0425]: cannot find value `i128` in module `self::shift_max`
[00:02:55]    --> libcore/num/wrapping.rs:115:29
[00:02:55]     |
[00:02:55] 115 |         sh_impl_unsigned! { $t, usize }
[00:02:55]     |                             ^^ not found in `self::shift_max`
[00:02:55] ...
[00:02:55] 125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
[00:02:55]     | -------------------------------------------------------------------- in this macro invocation
[00:02:59]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:02:59]    Compiling cmake v0.1.30
[00:02:59]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:03:04]    Compiling rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
---
[00:03:09] Caused by:
[00:03:09]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:09] warning: build failed, waiting for other jobs to finish...
[00:03:18] error: build failed
[00:03:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:18] expected success, got: exit code: 101
[00:03:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:18] travis_fold:end:stage0-std

[00:03:18] travis_time:end:stage0-std:start=1525798039369893925,finish=1525798065220596060,duration=25850702135


[00:03:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:18] Build completed unsuccessfully in 0:00:27
[00:03:18] Makefile:79: recipe for target 'tidy' failed
[00:03:18] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0606d361
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
