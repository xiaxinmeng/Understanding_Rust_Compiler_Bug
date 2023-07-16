plain

[00:03:40] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:40] tidy error: /checkout/src/librustc_data_structures/indexed_set.rs:212: trailing whitespace
[00:03:41] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:309: trailing whitespace
[00:03:41] tidy error: /checkout/src/librustc_mir/transform/qualify_consts.rs:685: trailing whitespace
[00:03:42] some tidy checks failed
[00:03:42] 
[00:03:42] 
[00:03:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:42] 
[00:03:42] 
[00:03:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:42] Build completed unsuccessfully in 0:00:46
[00:03:42] Build completed unsuccessfully in 0:00:46
[00:03:42] Makefile:79: recipe for target 'tidy' failed
[00:03:42] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002cd4a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:264f6840:start=1532015209114302543,finish=1532015209121844963,duration=7542420
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b59be8e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:185bf2a8
travis_time:start:185bf2a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fc0b61c
$ dmesg | grep -i kill
