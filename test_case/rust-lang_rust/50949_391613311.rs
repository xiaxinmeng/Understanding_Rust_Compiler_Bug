plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:18 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:19 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[00:58:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/tools/rls src/tools/rustfmt src/tools/miri src/tools/clippy
[00:58:21] Build completed unsuccessfully in 0:55:41
[00:58:21] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[00:58:21] {"rustfmt":"test-pass","miri":"test-pass","nomicon":"test-pass","reference":"test-pass","rls":"test-pass","book":"test-pass","clippy-driver":"test-pass","rust-by-example":"test-pass"}
[00:58:21] Verifying status of book...
[00:58:21] Verifying status of nomicon...
[00:58:21] Verifying status of reference...
