plain

[00:05:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:29] tidy error: /checkout/src/test/ui/cfg-arg-invalid-4.rs: incorrect license
[00:05:29] tidy error: /checkout/src/test/ui/cfg-arg-invalid-3.rs: incorrect license
[00:05:29] tidy error: /checkout/src/test/ui/cfg-attr-syntax-validation.rs: incorrect license
[00:05:29] tidy error: /checkout/src/test/ui/cfg-arg-invalid-2.rs: incorrect license
[00:05:29] tidy error: /checkout/src/test/ui/cfg-arg-invalid-5.rs: incorrect license
[00:05:29] tidy error: /checkout/src/test/ui/cfg-arg-invalid-1.rs: incorrect license
[00:05:31] some tidy checks failed
[00:05:31] 
[00:05:31] 
[00:05:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:31] 
[00:05:31] 
[00:05:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:31] Build completed unsuccessfully in 0:00:49
[00:05:31] Build completed unsuccessfully in 0:00:49
[00:05:31] make: *** [tidy] Error 1
[00:05:31] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e4933da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1ca8a02e:start=1536345892190143055,finish=1536345892198936453,duration=8793398
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0170b077
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01ae18dc
travis_time:start:01ae18dc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:008bdc82
$ dmesg | grep -i kill
