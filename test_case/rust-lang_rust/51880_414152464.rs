plain
[00:29:36]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:29:59] error: variable does not need to be mutable
[00:29:59]     --> librustc_typeck/check/mod.rs:4835:21
[00:29:59]      |
[00:29:59] 4835 |                 let mut generics = self.tcx.generics_of(def_id);
[00:29:59]      |                     |
[00:29:59]      |                     help: remove this `mut`
[00:29:59]      |
[00:29:59]      = note: `-D unused-mut` implied by `-D warnings`
[00:29:59]      = note: `-D unused-mut` implied by `-D warnings`
[00:29:59] 
[00:30:03] error: aborting due to previous error
[00:30:03] 
[00:30:03] error: Could not compile `rustc_typeck`.
[00:30:03] 
[00:30:03] Caused by:
[00:30:03]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_typeck librustc_typeck/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=910afcca4638babe -C extra-filename=-910afcca4638babe --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4701371fdd2d83ac.so --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-740445503e73846f.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-4ace312b04b1699b.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-e2d832ddcbc5834d.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-319acabc7334063e.so --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-4256aaadcf4213fe.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-09eaeb74e6af7d97.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-c2faafd836159cc9.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-088e8a2c200eaae6.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-cd6f328488af7b94/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-b967192cda86eebc/out` (exit code: 1)
[00:32:16] error: build failed
[00:32:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:32:16] expected success, got: exit code: 101
[00:32:16] expected success, got: exit code: 101
[00:32:16] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:32:16] travis_fold:end:stage1-rustc

[00:32:16] travis_time:end:stage1-rustc:start=1534708593665871474,finish=1534709257984018498,duration=664318147024


[00:32:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:32:16] Build completed unsuccessfully in 0:27:41
[00:32:16] Makefile:28: recipe for target 'all' failed
[00:32:16] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2f1ed6b8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:21c0fbee:start=1534709258886726248,finish=1534709258894159566,duration=7433318
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1233f9b2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:32222032
travis_time:start:32222032
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:158e8f90
$ dmesg | grep -i kill
