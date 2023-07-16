plain
travis_time:end:1066a020:start=1551522415120692013,finish=1551522498370369137,duration=83249677124
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:24:11] 
[00:24:11] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:24:11] 
[00:24:11] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C panic=abort -C debug-assertions=no -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:24:11] note: some of the compiler flags provided by cargo are hidden
[00:24:11] 
[00:24:11] error: Could not compile `compiler_builtins`.
[00:24:11] warning: build failed, waiting for other jobs to finish...
---
19956 ./src/llvm-project/llgo/third_party/gofrontend/libgo/go
19596 ./src/tools/rls/rls-analysis
travis_time:end:024f2f28:start=1551523958860694843,finish=1551523959478273171,duration=617578328
travis_fold:end:after_failure.1
travis_folib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23fa4226
$ dmesg | grep -i kill
