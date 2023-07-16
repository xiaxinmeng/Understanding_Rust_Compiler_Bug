plain
travis_time:end:136b434d:start=1551294267013841198,finish=1551294267999853429,duration=986012231
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:26:48] 
[00:26:48] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:26:48] 
[00:26:48] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib --crate-type rlib
[00:26:48] note: some of the compiler flags provided by cargo are hidden
[00:26:48] 
[00:26:48] error: Could not compile `test`.
[00:26:48] 
[00:26:48] 
[00:26:48] To learn more, run the command again with --verbose.
[00:26:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
[00:26:48] expected success, got: exit code: 101
[00:26:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:48] Build completed unsuccessfully in 0:21:54
/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0417b608
$ dmesg | grep -i kill
