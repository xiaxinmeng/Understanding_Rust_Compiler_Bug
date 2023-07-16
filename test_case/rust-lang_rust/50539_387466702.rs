plain
[00:03:22]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:23] error[E0547]: missing 'issue'
[00:03:23]    --> libcore/num/f32.rs:132:5
[00:03:23]     |
[00:03:23] 132 |     #[unstable(feature = "extra_log_consts")]
[00:03:23] 
[00:03:23] error[E0547]: missing 'issue'
[00:03:23]    --> libcore/num/f32.rs:140:5
[00:03:23]     |
[00:03:23]     |
[00:03:23] 140 |     #[unstable(feature = "extra_log_consts")]
[00:03:23] 
[00:03:23] error[E0547]: missing 'issue'
[00:03:23]    --> libcore/num/f64.rs:128:5
[00:03:23]     |
[00:03:23]     |
[00:03:23] 128 |     #[unstable(feature = "extra_log_consts")]
[00:03:23] 
[00:03:23] error[E0547]: missing 'issue'
[00:03:23]    --> libcore/num/f64.rs:136:5
[00:03:23]     |
[00:03:23]     |
[00:03:23] 136 |     #[unstable(feature = "extra_log_consts")]
[00:03:23] 
[00:03:26]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:26]    Compiling cmake v0.1.30
[00:03:26]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
---
[00:03:36] Caused by:
[00:03:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=fb1e36473ec4786e -C extra-filename=-fb1e36473ec4786e --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:36] warning: build failed, waiting for other jobs to finish...
[00:03:45] error: build failed
[00:03:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:45] expected success, got: exit code: 101
[00:03:45] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:45] travis_fold:end:stage0-std

[00:03:45] travis_time:end:stage0-std:start=1525797730516430972,finish=1525797757743978115,duration=27227547143


[00:03:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:45] Build completed unsuccessfully in 0:00:29
[00:03:45] Makefile:79: recipe for target 'tidy' failed
[00:03:45] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34322484
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
