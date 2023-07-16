plain
[01:36:37] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargotest", path: "src/tools/cargotest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 1.421
[01:36:37] testing https://github.com/servo/webrender
[01:36:37] Initialized empty Git repository in /checkout/obj/build/ct/webrender/.git/
[01:36:37] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:37:33] fatal: unable to access 'https://github.com/servo/webrender/': Could not resolve host: github.com
[01:37:33] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:37:33] 
[01:37:33] 
[01:37:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:37:33] expected success, got: exit code: 101
[01:37:33] expected success, got: exit code: 101
[01:37:33] 
[01:37:33] 
[01:37:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:37:33] Build completed unsuccessfully in 0:34:54
[01:37:33] make: *** [check-aux] Error 1
[01:37:33] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:287733f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3d22c29c:start=1533105732399760091,finish=1533105732405382182,duration=5622091
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10df0bf0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14e19370
travis_time:start:14e19370
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2cc61d5e
$ dmesg | grep -i kill
