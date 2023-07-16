plain
travis_time:end:0aa6650c:start=1540564611380012735,finish=1540564612438827965,duration=1058815230
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:13:05]    Compiling rustc_metadata_utils v0.0.0 (/checkout/src/librustc_metadata_utils)
[00:13:05]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:13:05]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:06]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:13:07] error: unused import: `MemoryAccess`
[00:13:07]   --> librustc_mir/interpret/memory.rs:33:14
[00:13:07]    |
[00:13:07] 33 |     Machine, MemoryAccess, AllocMap, MayLeak, ScalarMaybeUndef, AllocationExtra, ErrorHandled,
[00:13:07]    |
[00:13:07]    = note: `-D unused-imports` implied by `-D warnings`
[00:13:07] 
[00:13:20] error: aborting due to previous error
---
travis_time:end:1aaa46e5:start=1540565490221009293,finish=1540565490225237043,duration=4227750
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06b0cad3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:195ee6b0
travis_time:start:195ee6b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0191ddd1
$ dmesg | grep -i kill
