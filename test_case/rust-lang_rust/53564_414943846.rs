plain
[00:20:03] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:20:03] 
[00:20:03] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:20:03] 
[00:20:03] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:20:03] note: some of the compiler flags provided by cargo are hidden
[00:20:03] 
[00:20:03] error: Could not compile `core`.
[00:20:03] 
[00:20:03] 
[00:20:03] Caused by:
[00:20:03]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=aae624166adf9237 -C extra-filename=-aae624166adf9237 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 101)
[00:20:03] warning: build failed, waiting for other jobs to finish...
[00:20:07] error: build failed
[00:20:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:07] expected success, got: exit code: 101
[00:20:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:20:07] travis_fold:end:stage1-std

[00:20:07] travis_time:end:stage1-std:start=1534924113101168159,finish=1534924118630042329,duration=5528874170


[00:20:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:07] Build completed unsuccessfully in 0:15:37
[00:20:07] make: *** [all] Error 1
[00:20:07] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e46c0e8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
