plain
[00:03:01]    Compiling cc v1.0.15
[00:03:01]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:03:01]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:03:01]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:03:01] error: unexpected close delimiter: `}`
[00:03:01]     --> libcore/slice/mod.rs:1866:2
[00:03:01] 1866 | }}
[00:03:01]      |  ^
[00:03:01] 
[00:03:01] error: aborting due to previous error
[00:03:01] error: aborting due to previous error
[00:03:01] 
[00:03:01] error: Could not compile `core`.
[00:03:01] 
[00:03:01] Caused by:
[00:03:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=0d1ebef792b1d9ca -C extra-filename=-0d1ebef792b1d9ca --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:01] warning: build failed, waiting for other jobs to finish...
[00:03:08] error: build failed
[00:03:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:08] expected success, got: exit code: 101
[00:03:08] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:08] travis_fold:end:stage0-std

[00:03:08] travis_time:end:stage0-std:start=1526586698663571079,finish=1526586705877365739,duration=7213794660


[00:03:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:08] Build completed unsuccessfully in 0:00:08
[00:03:08] Makefile:79: recipe for target 'tidy' failed
[00:03:08] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15576cbd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
