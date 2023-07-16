plain
travis_time:end:0a809ead:start=1553509994966091590,finish=1553509997404379052,duration=2438287462
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:28:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:11] 
[00:28:11] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:28:11] 
[00:28:11] note: compiler flags: -Z external-macro-backtrace -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:28:11] note: some of the compiler flags provided by cargo are hidden
[00:28:11] 
[00:28:11] error: Could not compile `core`.
[00:28:11] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:0ddb9e6c:start=1553511703921524086,finish=1553511703926907045,duration=5382959
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b54c680
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:142bf2b8
travis_time:start:142bf2b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33c9beef
$ dmesg | grep -i kill
