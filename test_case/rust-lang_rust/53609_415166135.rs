plain

[00:04:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:26] tidy error: /checkout/src/librustc_mir/interpret/cast.rs:230: line longer than 100 chars
[00:04:26] tidy error: /checkout/src/librustc_mir/interpret/cast.rs:244: line longer than 100 chars
[00:04:26] tidy error: /checkout/src/librustc_mir/interpret/cast.rs:307: line longer than 100 chars
[00:04:28] some tidy checks failed
[00:04:28] 
[00:04:28] 
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] 
[00:04:28] 
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:00:51
[00:04:28] Build completed unsuccessfully in 0:00:51
[00:04:28] Makefile:79: recipe for target 'tidy' failed
[00:04:28] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0537082e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:058b3a3a:start=1534968844231861823,finish=1534968844240240223,duration=8378400
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13dfd45c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21188d3e
travis_time:start:21188d3e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3e0564e8
$ dmesg | grep -i kill
