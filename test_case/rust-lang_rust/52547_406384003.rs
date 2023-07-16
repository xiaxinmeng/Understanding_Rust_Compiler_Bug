plain
travis_time:start:tidy
tidy check
[00:03:55] * 553 error codes
[00:03:55] * highest error code: E0710
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:734: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1099: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1154: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1449: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1513: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1744: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:1780: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:2473: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:2521: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:2826: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:2871: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:3132: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:3185: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:3361: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libcore/num/mod.rs:3383: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libstd/f64.rs:222: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libstd/f64.rs:256: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libstd/f32.rs:246: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:55] tidy error: /checkout/src/libstd/f32.rs:280: mismatches to previous in: ["stability level", "since", "tracking issue"]
[00:03:56] some tidy checks failed
[00:03:56] 
[00:03:56] 
[00:03:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:56] 
[00:03:56] 
[00:03:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:56] Build completed unsuccessfully in 0:00:52
[00:03:56] Build completed unsuccessfully in 0:00:52
[00:03:56] Makefile:79: recipe for target 'tidy' failed
[00:03:56] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f0f928e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:018ad95e:start=1532027103014669189,finish=1532027103021487372,duration=6818183
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2195a1a7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00a1d3d0
travis_time:start:00a1d3d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0342d4a1
$ dmesg | grep -i kill
