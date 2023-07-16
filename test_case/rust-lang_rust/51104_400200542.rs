plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:51 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:52 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:53 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:54 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:55 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[00:58:01] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:589:17
[00:58:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/tools/rls src/tools/rustfmt src/tools/miri src/tools/clippy
[00:58:01] Build completed unsuccessfully in 0:55:04
[00:58:01] {"nomicon":"test-pass","rust-by-example":"test-pass","miri":"test-pass","clippy-driver":"test-pass","book":"test-pass","rls":"test-pass","reference":"test-pass","rustfmt":"test-pass"}
[00:58:01] Verifying status of book...
---
travis_time:end:2b614c90:start=1529996220523411078,finish=1529996220531583776,duration=8172698
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2685269d
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:063eaa32
$ dmesg | grep -i kill
