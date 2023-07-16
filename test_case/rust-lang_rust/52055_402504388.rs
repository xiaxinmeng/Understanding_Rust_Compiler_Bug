plain

[00:04:37] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:37] tidy error: /checkout/src/librustc_codegen_llvm/back/link.rs:819: line longer than 100 chars
[00:04:39] some tidy checks failed
[00:04:39] 
[00:04:39] 
[00:04:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:39] 
[00:04:39] 
[00:04:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:39] Build completed unsuccessfully in 0:01:40
[00:04:39] Build completed unsuccessfully in 0:01:40
[00:04:39] make: *** [tidy] Error 1
[00:04:39] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17558dc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:134c0374:start=1530716236866009192,finish=1530716236873579748,duration=7570556
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10876add
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0500833b
$ dmesg | grep -i kill
