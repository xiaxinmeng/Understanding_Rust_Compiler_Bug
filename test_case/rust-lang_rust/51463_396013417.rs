plain

[00:05:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:14] tidy error: /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:63: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/test/compile-fail/associated-types-unsized.rs:17: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/test/compile-fail/trait-bounds-not-on-bare-trait.rs:18: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/test/compile-fail/issue-24446.rs:14: line longer than 100 chars
[00:05:14] tidy error: /checkout/src/libsyntax/parse/parser.rs:662: line longer than 100 chars
[00:05:16] some tidy checks failed
[00:05:16] 
[00:05:16] 
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:16] 
[00:05:16] 
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:16] Build completed unsuccessfully in 0:01:52
[00:05:16] Build completed unsuccessfully in 0:01:52
[00:05:16] Makefile:79: recipe for target 'tidy' failed
[00:05:16] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02efa194
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:224b1bc4:start=1528595389343086734,finish=1528595389350678280,duration=7591546
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12127b60
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02119d68
$ dmesg | grep -i kill
