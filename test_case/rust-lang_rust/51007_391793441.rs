plain
[00:21:47]    Compiling rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:22:38] error: Could not compile `core`.
[00:22:38] 
[00:22:38] Caused by:
[00:22:38]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=9b2cf3afb370a473 -C extra-filename=-9b2cf3afb370a473 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (signal: 11, SIGSEGV: invalid memory reference)
[00:22:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:22:38] expected success, got: exit code: 101
[00:22:38] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:22:38] travis_fold:end:stage1-std

[00:22:38] travis_time:end:stage1-std:start=1527182358876959248,finish=1527182424845474199,duration=65968514951


[00:22:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:38] Build completed unsuccessfully in 0:18:05
[00:22:38] Makefile:28: recipe for target 'all' failed
[00:22:38] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0297630d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
