plain
tidy check
[00:05:21] * 566 error codes
[00:05:21] * highest error code: E0721
[00:05:22] * 238 features
[00:05:22] invalid source: "git+https://github.com/rust-lang-nursery/rust-clippy?rev=f5d868c9edfc6c2a9310d564a2f738bec67dfd6b#f5d868c9edfc6c2a9310d564a2f738bec67dfd6b"
[00:05:22] invalid source: "git+https://github.com/rust-lang-nursery/rust-clippy?rev=f5d868c9edfc6c2a9310d564a2f738bec67dfd6b#f5d868c9edfc6c2a9310d564a2f738bec67dfd6b"
[00:05:22] some tidy checks failed
[00:05:22] 
[00:05:22] 
[00:05:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:22] 
[00:05:22] 
[00:05:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:22] Build completed unsuccessfully in 0:00:54
[00:05:22] Build completed unsuccessfully in 0:00:54
[00:05:22] Makefile:79: recipe for target 'tidy' failed
[00:05:22] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:063ce168
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 17 18:04:12 UTC 2018
---
travis_time:end:11871696:start=1545069853089752249,finish=1545069853097143790,duration=7391541
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002dcb40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b36e060
travis_time:start:0b36e060
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ce59b90
$ dmesg | grep -i kill
