plain
travis_time:end:202bf90c:start=1546730289006703010,finish=1546730291405854971,duration=2399151961
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:03:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:54] tidy error: /checkout/src/libcore/iter/adapters/flatten.rs:231: line longer than 100 chars
[00:03:54] tidy error: /checkout/src/libcore/iter/adapters/flatten.rs:234: line longer than 100 chars
40496 ./src/llvm-emscripten/lib/Target
37784 ./src/tools/lldb/www
36020 ./src/tools/clang/lib
34948 ./src/llvm/test/tools
