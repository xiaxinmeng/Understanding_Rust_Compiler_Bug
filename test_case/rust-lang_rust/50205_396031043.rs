plain
[01:42:47] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "cargotest", path: "src/tools/cargotest", mode: ToolStd, is_ext_tool: false, extra_features: [] } -- 1.618
[01:42:47] testing https://github.com/servo/webrender
[01:42:47] Initialized empty Git repository in /checkout/obj/build/ct/webrender/.git/
[01:42:47] fatal: Could not parse object '57250b2b8fa63934f80e5376a29f7dcb3f759ad6'.
[01:42:50] fatal: unable to access 'https://github.com/servo/webrender/': Failed to connect to github.com port 443: No route to host
[01:42:50] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:42:50] 
[01:42:50] 
[01:42:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:42:50] expected success, got: exit code: 101
[01:42:50] expected success, got: exit code: 101
[01:42:50] 
[01:42:50] 
[01:42:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:42:50] Build completed unsuccessfully in 0:35:22
[01:42:50] make: *** [check-aux] Error 1
[01:42:50] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e659060
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02f52f82:start=1528619044852237416,finish=1528619044858472859,duration=6235443
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3a28a2d8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17aaa38e
$ dmesg | grep -i kill
