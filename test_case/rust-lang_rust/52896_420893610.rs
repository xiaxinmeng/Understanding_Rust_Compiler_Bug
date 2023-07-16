plain
[00:08:03]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:08:04] error: This node does not have a stability attribute
[00:08:04]    --> libproc_macro/diagnostic.rs:130:31
[00:08:04]     |
[00:08:04] 130 |     pub fn children(&self) -> impl Iterator<Item = &Diagnostic> {
[00:08:04] 
[00:08:04] error: aborting due to previous error
[00:08:04] 
[00:08:04] error: Could not compile `proc_macro`.
[00:08:04] error: Could not compile `proc_macro`.
[00:08:04] 
[00:08:04] Caused by:
[00:08:04]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name proc_macro libproc_macro/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=87919c9c23cf3652 -C extra-filename=-87919c9c23cf3652 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05301c67193a930e.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ac364f854372fe.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-0651ffc5a9129db1.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-47b99ffec2efbd05.so` (exit code: 1)
[00:08:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:04] expected success, got: exit code: 101
[00:08:04] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:08:04] travis_fold:end:stage0-rustc

[00:08:04] travis_time:end:stage0-rustc:start=1536818436003034068,finish=1536818574380296229,duration=138377262161


[00:08:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:04] Build completed unsuccessfully in 0:03:17
[00:08:04] make: *** [all] Error 1
[00:08:04] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00073bbd
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:128be956:start=1536818575070752570,finish=1536818575075771607,duration=5019037
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30eb2508
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0970f98c
travis_time:start:0970f98c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f8ea352
$ dmesg | grep -i kill
