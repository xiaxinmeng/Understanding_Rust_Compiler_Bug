plain
[00:03:47]    Compiling cc v1.0.15
[00:03:47]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:47]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:47]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:50] error[E0541]: unknown meta item 'since'
[00:03:50]     --> libcore/cell.rs:1145:47
[00:03:50]      |
[00:03:50] 1145 |     #[unstable(feature = "refcell_map_split", since = "0")]
[00:03:50] 
[00:03:50] 
[00:03:50] error[E0541]: unknown meta item 'since'
[00:03:50]     --> libcore/cell.rs:1231:47
[00:03:50]      |
[00:03:50] 1231 |     #[unstable(feature = "refcell_map_split", since = "0")]
[00:03:50] 
[00:03:52]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:52]    Compiling cmake v0.1.30
[00:03:52]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
[00:04:00] Caused by:
[00:04:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=e9cdce497aae9e81 -C extra-filename=-e9cdce497aae9e81 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:04:00] warning: build failed, waiting for other jobs to finish...
[00:04:11] error: build failed
[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:11] expected success, got: exit code: 101
[00:04:11] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:04:11] travis_fold:end:stage0-std

[00:04:11] travis_time:end:stage0-std:start=1528609189357781357,finish=1528609213901463650,duration=24543682293


[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:11] Build completed unsuccessfully in 0:00:25
[00:04:11] Makefile:79: recipe for target 'tidy' failed
[00:04:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:28c73509
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fc341af:start=1528609214449490020,finish=1528609214457192995,duration=7702975
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bfb0b72
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d561cc8
$ dmesg | grep -i kill
