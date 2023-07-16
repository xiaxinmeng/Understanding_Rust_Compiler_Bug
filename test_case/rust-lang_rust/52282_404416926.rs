plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:09:07] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:09:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/tools/rls src/tools/rustfmt src/tools/miri src/tools/clippy
[01:09:07] Build completed unsuccessfully in 1:06:37
[01:09:07] {"rustfmt":"test-pass","rls":"test-pass","reference":"test-pass","book":"test-pass","rust-by-example":"test-pass","nomicon":"test-fail","miri":"test-pass","clippy-driver":"test-fail"}
[01:09:07] Verifying status of book...
---
travis_time:end:19523df9:start=1531380505204666365,finish=1531380505210560956,duration=5894591
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:048c8304
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0178de98
$ dmesg | grep -i kill
