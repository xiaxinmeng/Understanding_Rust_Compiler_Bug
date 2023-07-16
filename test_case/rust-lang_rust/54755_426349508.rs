plain

[00:04:37] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:37] tidy error: /checkout/src/libstd/primitive_docs.rs:912: trailing whitespace
[00:04:37] tidy error: /checkout/src/libstd/primitive_docs.rs:914: trailing whitespace
[00:04:37] tidy error: /checkout/src/libstd/primitive_docs.rs:917: trailing whitespace
[00:04:37] tidy error: /checkout/src/libstd/primitive_docs.rs:933: trailing whitespace
[00:04:37] tidy error: /checkout/src/libstd/primitive_docs.rs:940: trailing whitespace
[00:04:38] some tidy checks failed
[00:04:38] 
[00:04:38] 
[00:04:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:38] 
[00:04:38] 
[00:04:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:38] Build completed unsuccessfully in 0:00:47
[00:04:38] Build completed unsuccessfully in 0:00:47
[00:04:38] Makefile:79: recipe for target 'tidy' failed
[00:04:38] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e5bb186
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1a6b3ef0:start=1538498791770823448,finish=1538498791776442676,duration=5619228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04f6b603
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d5b4054
travis_time:start:0d5b4054
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0342a099
$ dmesg | grep -i kill
