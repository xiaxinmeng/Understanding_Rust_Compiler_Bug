plain

[00:04:12] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:13] tidy error: /checkout/src/librustc/traits/auto_trait.rs:361: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/traits/auto_trait.rs:419: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/traits/auto_trait.rs:433: line longer than 100 chars
[00:04:13] tidy error: /checkout/src/librustc/traits/auto_trait.rs:437: line longer than 100 chars
[00:04:14] some tidy checks failed
[00:04:14] 
[00:04:14] 
[00:04:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:14] 
[00:04:14] 
[00:04:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:14] Build completed unsuccessfully in 0:00:47
[00:04:14] Build completed unsuccessfully in 0:00:47
[00:04:14] make: *** [tidy] Error 1
[00:04:14] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e26a3ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:29e38c96:start=1533235864116438005,finish=1533235864122468000,duration=6029995
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12e55cb0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:355bcd8a
travis_time:start:355bcd8a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17b23443
$ dmesg | grep -i kill
