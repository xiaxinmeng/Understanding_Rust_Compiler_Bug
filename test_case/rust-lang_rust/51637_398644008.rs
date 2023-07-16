plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:06:47] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[01:06:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
[01:06:47] Build completed unsuccessfully in 1:03:56
travis_time:end:02fa56ac:start=1529473566422106043,finish=1529477574469448976,duration=4008047342933

---
travis_time:end:029f5410:start=1529477575085227113,finish=1529477575092833958,duration=7606845
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13433aa8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:36e8137c
$ dmesg | grep -i kill
