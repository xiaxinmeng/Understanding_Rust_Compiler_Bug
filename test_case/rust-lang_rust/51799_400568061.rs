plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:07:10] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[01:07:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/tools/rls src/tools/rustfmt src/tools/miri src/tools/clippy
[01:07:10] Build completed unsuccessfully in 1:04:20
[01:07:10] {"miri":"test-pass","clippy-driver":"test-pass","rust-by-example":"test-pass","rustfmt":"test-pass","book":"test-pass","reference":"test-pass","rls":"test-pass","nomicon":"test-pass"}
[01:07:10] Verifying status of book...
---
travis_time:end:06770e00:start=1530083686967287405,finish=1530083686972872661,duration=5585256
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e00674c
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01a58d38
$ dmesg | grep -i kill
