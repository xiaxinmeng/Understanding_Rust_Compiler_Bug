plain
travis_time:end:08c98872:start=1550919838047717566,finish=1550919933563927792,duration=95516210226
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:24:40] 
[00:24:40] note: rustc 1.34.0-dev running on x86_64-unknown-linux-gnu
[00:24:40] 
[00:24:40] note: compiler flags: -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib --crate-type rlib
[00:24:40] note: some of the compiler flags provided by cargo are hidden
[00:24:40] 
[00:24:40] error: Could not compile `std`.
[00:24:40] 
---
175796 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
156160 ./src/llvm-project/clang
155960 ./obj/build/bootstrap/debug/incremental
141180 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141176 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9r6u681w5-sewl9m-1wm2zcqodn409
138684 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
135904 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
123516 ./src/llvm-project/llvm/test/CodeGen
108528 ./src/llvm-project/lldb
---
travis_time:end:1d7c2c10:start=1550921423902911470,finish=1550921423907610487,duration=4699017
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1187a373
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0201d80e
travis_time:start:0201d80e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01356a9e
$ dmesg | grep -i kill
