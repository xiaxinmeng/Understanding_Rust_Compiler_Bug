plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:02fa0e3a
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:08:46] Dist analysis
[01:08:46] image_src: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/save-analysis", dst: "/checkout/obj/build/tmp/dist/rust-analysis-nightly-x86_64-unknown-linux-gnu-image/lib/rustlib/x86_64-unknown-linux-gnu/analysis"
[01:08:49] Dist src
[01:09:01] Create plain source tarball
[01:10:58] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:12:51] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:14:43] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:14:43] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[01:14:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:14:43] Build completed unsuccessfully in 1:08:38
travis_time:end:1ee8ca06:start=1529559571418170013,finish=1529564054805697849,duration=4483387527836

