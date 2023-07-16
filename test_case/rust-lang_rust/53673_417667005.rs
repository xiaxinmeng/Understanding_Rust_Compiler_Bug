plain

[00:04:34] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:34] tidy error: /checkout/src/librustc_codegen_llvm/back/lto.rs: missing trailing newline
[00:04:36] Dependencies not on the whitelist:
[00:04:36] * memmap 
[00:04:36] some tidy checks failed
[00:04:36] 
[00:04:36] 
[00:04:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:36] 
[00:04:36] 
[00:04:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:36] Build completed unsuccessfully in 0:00:50
[00:04:36] Build completed unsuccessfully in 0:00:50
[00:04:36] make: *** [tidy] Error 1
[00:04:36] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c3818ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04a72806:start=1535722178906869167,finish=1535722178915041066,duration=8171899
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:211706f3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21c8d14e
travis_time:start:21c8d14e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:011cb500
$ dmesg | grep -i kill
