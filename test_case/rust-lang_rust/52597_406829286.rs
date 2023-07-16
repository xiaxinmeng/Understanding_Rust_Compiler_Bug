plain

[00:03:42] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:43] tidy error: /checkout/src/librustc_codegen_llvm/mir/place.rs:464: line longer than 100 chars
[00:03:44] some tidy checks failed
[00:03:44] 
[00:03:44] 
[00:03:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:44] 
[00:03:44] 
[00:03:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:44] Build completed unsuccessfully in 0:00:48
[00:03:44] Build completed unsuccessfully in 0:00:48
[00:03:44] make: *** [tidy] Error 1
[00:03:44] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26927b50
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:18f4906e:start=1532214629368554868,finish=1532214629377598960,duration=9044092
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0020ae18
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:012ee5c0
travis_time:start:012ee5c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:233ed93e
$ dmesg | grep -i kill
