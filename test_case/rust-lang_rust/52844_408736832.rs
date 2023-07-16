plain

[00:07:41] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] } -- 49.277
[00:11:11] [TIMING] PlainSourceTarball -- 341.526
[00:11:11] Dist src
[00:11:11] thread 'main' panicked at 'could not read dir "/checkout/src/libstd_unicode": Os { code: 2, kind: NotFound, message: "No such file or directory" }', bootstrap/lib.rs:1248:25
[00:11:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:11:11] Build completed unsuccessfully in 0:08:32
travis_time:end:01e647b4:start=1532920615442519425,finish=1532921287210773916,duration=671768254491

---
travis_time:end:010ce717:start=1532921287698134608,finish=1532921287703174524,duration=5039916
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:012c6fd2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fbae336
travis_time:start:0fbae336
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02460370
$ dmesg | grep -i kill
