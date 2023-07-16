plain
Building stage2 tool cargo (x86_64-unknown-linux-gnu)
[01:15:05]  Downloading crates ...
[01:15:25] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:15:45] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:16:05] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:16:05] Caused by:
[01:16:05]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:16:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:16:05] expected success, got: exit code: 101
[01:16:05] expected success, got: exit code: 101
[01:16:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:16:05] Build completed unsuccessfully in 0:17:03
[01:16:05] make: *** [check-aux] Error 1
[01:16:05] Makefile:60: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c38bd1c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec  8 07:34:51 UTC 2018
---
travis_time:end:3416b510:start=1544254492050404919,finish=1544254492059122254,duration=8717335
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:345dd1c9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:28b165c6
travis_time:start:28b165c6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:164f442c
$ dmesg | grep -i kill
