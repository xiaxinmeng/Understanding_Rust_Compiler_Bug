plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:17:53] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[01:17:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host mipsel-unknown-linux-gnu --target mipsel-unknown-linux-gnu
[01:17:53] Build completed unsuccessfully in 1:14:43
travis_time:end:01375800:start=1529126347263326961,finish=1529131020968299183,duration=4673704972222

---
travis_time:end:1b0af65c:start=1529131021599621814,finish=1529131021608246093,duration=8624279
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02ecdc00
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e40e54
$ dmesg | grep -i kill
