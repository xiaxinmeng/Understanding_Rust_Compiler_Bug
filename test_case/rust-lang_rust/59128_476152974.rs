plain
travis_time:end:14206ea2:start=1553509153952069577,finish=1553509228879904662,duration=74927835085
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:55:56]    Compiling parking_lot_core v0.4.0
[00:56:00]    Compiling tempfile v3.0.5
[00:56:02]    Compiling parking_lot v0.7.1
[00:56:03]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:56:04] error[E0433]: failed to resolve: use of undeclared type or module `HumanReadableErrorType`
[00:56:04]     |
[00:56:04]     |
[00:56:04] 260 |             Some("human") => ErrorOutputType::HumanReadable(HumanReadableErrorType::Default(color)),
[00:56:04]     |                                                             ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `HumanReadableErrorType`
[00:56:04] 
[00:56:04] error[E0433]: failed to resolve: use of undeclared type or module `HumanReadableErrorType`
[00:56:04]     |
[00:56:04]     |
[00:56:04] 263 |                 json_rendered: HumanReadableErrorType::Default(color),
[00:56:04]     |                                ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `HumanReadableErrorType`
[00:56:04] 
[00:56:04] error[E0433]: failed to resolve: use of undeclared type or module `HumanReadableErrorType`
[00:56:04]     |
[00:56:04]     |
[00:56:04] 267 |                 json_rendered: HumanReadableErrorType::Default(color),
[00:56:04]     |                                ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `HumanReadableErrorType`
[00:56:04] 
[00:56:04] error[E0433]: failed to resolve: use of undeclared type or module `HumanReadableErrorType`
[00:56:04]     |
[00:56:04]     |
[00:56:04] 269 |             Some("short") => ErrorOutputType::HumanReadable(HumanReadableErrorType::Short(color)),
[00:56:04]     |                                                             ^^^^^^^^^^^^^^^^^^^^^^ use of undeclared type or module `HumanReadableErrorType`
[00:56:04] 
[00:56:07] error[E0599]: no variant named `Short` found for type `rustc::session::config::ErrorOutputType` in the current scope
[00:56:07]     |
[00:56:07]     |
[00:56:07] 322 |         ErrorOutputType::Short(color_config) => Box::new(
[00:56:07]     |                          ^^^^^ variant not found in `rustc::session::config::ErrorOutputType`
[00:56:09] error: aborting due to 6 previous errors
[00:56:09] 
[00:56:09] Some errors occurred: E0308, E0433, E0599.
[00:56:09] For more information about an error, try `rustc --explain E0308`.
---
19956 ./src/llvm-project/llgo/third_party/gofrontend/libgo/go
19640 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
travis_time:end:02d26633:start=1553512607823526651,finish=1553512608458734203,duration=635207552
travis_fold:end:after_failure.1
travis_fold:start:after_failure-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0709e2e8
$ dmesg | grep -i kill
