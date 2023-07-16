plain
travis_time:end:0a4510a0:start=1544535642361679425,finish=1544535700833933589,duration=58472254164
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:09] 
[00:04:09] error: This node does not have a stability attribute
[00:04:09]     --> src/libcore/num/mod.rs:4866:5
[00:04:09]      |
[00:04:09] 4866 | /     pub fn kind(self) -> IntErrorKind {
[00:04:09] 4867 | |         self.kind
[00:04:09]      | |_____^
[00:04:09] 
[00:04:10] error: aborting due to previous error
[00:04:10] 
---
184272 ./obj/build/cache/2018-10-30
153272 ./src/tools/clang
150704 ./obj/build/bootstrap/debug/incremental
135104 ./obj/build/bootstrap/debug/incremental/bootstrap-2pgjvb3usndhe
135100 ./obj/build/bootstrap/debug/incremental/bootstrap-2pgjvb3usndhe/s-f7hpylm673-vezc7z-e1ql6nnu93qh
134556 ./.git/modules/src
115356 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
---
travis_time:end:0094e783:start=1544535960749111580,finish=1544535960756219755,duration=7108175
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03d0d0a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01736f7b
travis_time:start:01736f7b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02f33083
$ dmesg | grep -i kill
