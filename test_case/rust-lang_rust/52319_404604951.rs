plain

[00:04:25] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:25] tidy error: /checkout/src/libsyntax/parse/parser.rs:6225: line longer than 100 chars
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:00:46
[00:04:26] Build completed unsuccessfully in 0:00:46
[00:04:26] Makefile:79: recipe for target 'tidy' failed
[00:04:26] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:29f22201
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0a179cd8:start=1531419239276042629,finish=1531419239284730691,duration=8688062
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1987f593
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08720b83
$ dmesg | grep -i kill
