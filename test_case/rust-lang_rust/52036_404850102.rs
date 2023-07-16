plain
[00:33:10] Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:33:10] Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:33:10] Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:33:10] Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:33:10] thread 'main' panicked at 'File::open(stamp) failed with No such file or directory (os error 2)', bootstrap/lib.rs:1108:12
[00:33:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:33:10] Build completed unsuccessfully in 0:29:36
[00:33:10] Build completed unsuccessfully in 0:29:36
[00:33:10] make: *** [all] Error 1
[00:33:10] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04942aa0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08bffda6:start=1531492167650324644,finish=1531492167656424917,duration=6100273
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:010bae86
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06cb5830
$ dmesg | grep -i kill
