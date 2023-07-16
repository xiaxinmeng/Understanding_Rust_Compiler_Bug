plain
[00:02:56]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:56]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:56]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:02:59]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:02:59] error[E0541]: unknown meta item 'version'
[00:02:59]     --> libcore/fmt/mod.rs:1387:43
[00:02:59]      |
[00:02:59] 1387 |     #[stable(feature = "fmt_flags_align", version = "1.28.0")]
[00:02:59] 
[00:03:03]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:03]    Compiling cmake v0.1.30
[00:03:03]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
[00:03:10] Caused by:
[00:03:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=0d1ebef792b1d9ca -C extra-filename=-0d1ebef792b1d9ca --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:10] warning: build failed, waiting for other jobs to finish...
[00:03:21] error: build failed
[00:03:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:21] expected success, got: exit code: 101
[00:03:21] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:21] travis_fold:end:stage0-std

[00:03:21] travis_time:end:stage0-std:start=1527325845822004388,finish=1527325871017376103,duration=25195371715


[00:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:21] Build completed unsuccessfully in 0:00:26
[00:03:21] Makefile:79: recipe for target 'tidy' failed
[00:03:21] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0af8454a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
