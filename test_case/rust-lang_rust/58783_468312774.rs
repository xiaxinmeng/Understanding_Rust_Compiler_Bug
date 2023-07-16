plain
travis_time:end:30319caa:start=1551365512097169172,finish=1551365657099576484,duration=145002407312
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:27:02] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:02] 
[00:27:02] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:27:02] 
[00:27:02] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:02] note: some of the compiler flags provided by cargo are hidden
[00:27:02] 
[00:27:02] error: Could not compile `rustc_apfloat`.
[00:27:02] warning: build failed, waiting for other jobs to finish...
---
145228 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
145224 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142404 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
141172 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141168 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9wvmfulbl-lx0qp8-okq6jwyp562e
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
78764 ./.git/modules
---
travis_time:end:010c1b2c:start=1551367295329126399,finish=1551367295333786765,duration=4660366
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2042fa48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
t
