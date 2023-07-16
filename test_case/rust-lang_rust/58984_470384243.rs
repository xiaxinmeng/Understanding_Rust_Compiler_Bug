plain
travis_time:end:2073d77c:start=1551931089739632924,finish=1551931090630644953,duration=891012029
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:54:55] 
[00:54:55] error[E0308]: mismatched types
[00:54:55]    --> src/librustdoc/test.rs:275:70
[00:54:55]     |
[00:54:55] 275 |         let diagnostic_handler = errors::Handler::with_emitter(true, false, box emitter);
[00:54:55]     |
[00:54:55]     = note: expected type `std::option::Option<usize>`
[00:54:55]                found type `bool`
[00:54:55] 
[00:54:55] 
[00:54:55] error[E0308]: mismatched types
[00:54:55]    --> src/librustdoc/test.rs:427:52
[00:54:55]     |
[00:54:55] 427 |         let handler = Handler::with_emitter(false, false, box emitter);
[00:54:55]     |
[00:54:55]     = note: expected type `std::option::Option<usize>`
[00:54:55]                found type `bool`
[00:54:55] 
---
19256 ./src/llvm-project/lldb/www/cpp_reference/html
18804 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen
travis_time:end:0042fd82:start=1551934397473119982,finish=1551934398214617514,duration=741497532
travis_fold:end:after_failure.1
travis_fold:start:after_failure.dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a85f0c
$ dmesg | grep -i kill
