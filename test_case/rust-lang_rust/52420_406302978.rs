plain

[00:03:39] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:39] tidy error: /checkout/src/liballoc/string.rs:994: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/liballoc/collections/linked_list.rs:1320: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/liballoc/collections/vec_deque.rs:606: line longer than 100 chars
[00:03:39] tidy error: /checkout/src/liballoc/vec.rs:566: line longer than 100 chars
[00:03:41] some tidy checks failed
[00:03:41] 
[00:03:41] 
[00:03:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:41] 
[00:03:41] 
[00:03:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:41] Build completed unsuccessfully in 0:00:45
[00:03:41] Build completed unsuccessfully in 0:00:45
[00:03:41] Makefile:79: recipe for target 'tidy' failed
[00:03:41] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00dfee99
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:005a7d82:start=1532011648424021661,finish=1532011648430689459,duration=6667798
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26d0c12a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b05c20b
travis_time:start:0b05c20b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08d4dcf9
$ dmesg | grep -i kill
