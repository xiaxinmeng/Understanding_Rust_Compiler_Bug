plain
tidy check
[00:04:40] * 555 error codes
[00:04:40] * highest error code: E0712
[00:04:40] * 222 features
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-in-match.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-maybe-bad-lifetime.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-in-while.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-bad-lifetime.nll.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-opt-init.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-bad-lifetime.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-bad-type.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-opt-init.nll.stderr"
[00:04:41] Stray file with UI testing output: "/checkout/src/test/ui/catch/catch-maybe-bad-lifetime.nll.stderr"
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:53
[00:04:41] Build completed unsuccessfully in 0:00:53
[00:04:41] make: *** [tidy] Error 1
[00:04:41] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:18f85126
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02761ba0:start=1534722499155130025,finish=1534722499160892107,duration=5762082
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a40488b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:132c4674
travis_time:start:132c4674
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1150b804
$ dmesg | grep -i kill
