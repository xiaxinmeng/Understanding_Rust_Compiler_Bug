plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:08:55] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:08:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:08:55] Build completed unsuccessfully in 0:15:18
[01:08:55] Makefile:60: recipe for target 'check-aux' failed
[01:08:55] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003a6be4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fcf03c6:start=1530947143518061999,finish=1530947143523520981,duration=5458982
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:024074f3
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07c2f1d8
$ dmesg | grep -i kill
