plain

[00:04:32] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:32] tidy error: /checkout/src/librustc_codegen_llvm/back/linker.rs:1218: trailing whitespace
[00:04:32] tidy error: /checkout/src/librustc_codegen_llvm/back/linker.rs:1232: trailing whitespace
[00:04:34] some tidy checks failed
[00:04:34] 
[00:04:34] 
[00:04:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:34] 
[00:04:34] 
[00:04:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:34] Build completed unsuccessfully in 0:00:49
[00:04:34] Build completed unsuccessfully in 0:00:49
[00:04:34] make: *** [tidy] Error 1
[00:04:34] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01c003d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:047ab3fa:start=1537535545822976725,finish=1537535545829289459,duration=6312734
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a0fb95f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16fb6cee
travis_time:start:16fb6cee
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04a410e0
$ dmesg | grep -i kill
