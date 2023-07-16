plain

[00:04:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:45] tidy error: /checkout/src/test/compile-fail/borrowck/borrowck-slice-pattern-element-loan.rs:28: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/test/compile-fail/borrowck/borrowck-slice-pattern-element-loan.rs:44: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/test/compile-fail/borrowck/borrowck-slice-pattern-element-loan.rs:61: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/test/compile-fail/borrowck/borrowck-slice-pattern-element-loan.rs:79: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/test/compile-fail/borrowck/borrowck-slice-pattern-element-loan.rs:88: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:406: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:407: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:408: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:409: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:426: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/path_utils.rs:439: line longer than 100 chars
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:43
[00:04:47] Build completed unsuccessfully in 0:01:43
[00:04:47] make: *** [tidy] Error 1
[00:04:47] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04c53c49
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04f91e01:start=1530262771455773539,finish=1530262771462433506,duration=6659967
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029ac4eb
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11b1563b
$ dmesg | grep -i kill
