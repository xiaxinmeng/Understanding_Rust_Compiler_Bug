plain

[00:04:29] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1225: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1228: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1231: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1234: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1237: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1240: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1243: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1246: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1249: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1252: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1255: tab character
[00:04:29] tidy error: /checkout/src/librustc_codegen_llvm/intrinsic.rs:1258: tab character
[00:04:30] some tidy checks failed
[00:04:30] 
[00:04:30] 
[00:04:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:30] 
[00:04:30] 
[00:04:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:30] Build completed unsuccessfully in 0:00:48
[00:04:30] Build completed unsuccessfully in 0:00:48
[00:04:30] make: *** [tidy] Error 1
[00:04:30] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06237100
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02f5999c:start=1539011920214262156,finish=1539011920218735286,duration=4473130
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0271149c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:150a72bc
travis_time:start:150a72bc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d4e2cba
$ dmesg | grep -i kill
