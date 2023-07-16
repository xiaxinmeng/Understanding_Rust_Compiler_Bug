plain

[00:05:04] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] } -- 47.643
[00:10:41] [TIMING] PlainSourceTarball -- 479.451
[00:10:41] Dist src
[00:10:41] thread 'main' panicked at 'could not read dir "/checkout/src/libbacktrace": Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/bootstrap/lib.rs:1299:25
[00:10:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:10:41] Build completed unsuccessfully in 0:08:50
travis_time:end:1799dba0:start=1545462233422154972,finish=1545462875091581815,duration=641669426843
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:086565a0:start=1545462876120625988,finish=1545462876128908448,duration=8282460
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f564a88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0566c22c
travis_time:start:0566c22c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1961a48c
$ dmesg | grep -i kill
