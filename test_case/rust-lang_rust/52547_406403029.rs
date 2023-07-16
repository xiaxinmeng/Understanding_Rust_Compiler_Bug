plain
travis_time:start:tidy
tidy check
[00:04:00] * 553 error codes
[00:04:00] * highest error code: E0710
[00:04:00] tidy error: /checkout/src/libcore/num/mod.rs:2521: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:00] tidy error: /checkout/src/libcore/num/mod.rs:2826: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:00] tidy error: /checkout/src/libcore/num/mod.rs:2871: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:00] tidy error: /checkout/src/libcore/num/mod.rs:3132: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:00] tidy error: /checkout/src/libcore/num/mod.rs:3383: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:00] tidy error: /checkout/src/libstd/f64.rs:222: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:04:01] some tidy checks failed
[00:04:01] 
[00:04:01] 
[00:04:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:01] 
[00:04:01] 
[00:04:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:01] Build completed unsuccessfully in 0:00:53
[00:04:01] Build completed unsuccessfully in 0:00:53
[00:04:01] Makefile:79: recipe for target 'tidy' failed
[00:04:01] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1cf3816e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:064a2f9e:start=1532031491004360368,finish=1532031491011799493,duration=7439125
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04745aa9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003243c0
travis_time:start:003243c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06ab0efa
$ dmesg | grep -i kill
