plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:06:53] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:06:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
[01:06:53] Build completed unsuccessfully in 1:03:59
travis_time:end:153cd66e:start=1529645263675370664,finish=1529649277373324761,duration=4013697954097

---
travis_time:end:056c4936:start=1529649277799708439,finish=1529649277805154379,duration=5445940
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0db27cd8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04eedbe2
$ dmesg | grep -i kill
