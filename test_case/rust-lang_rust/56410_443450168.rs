plain
travis_time:end:1c3ab440:start=1543686799054107223,finish=1543686801424514854,duration=2370407631
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:14] 
[00:58:14] running 119 tests
[00:58:17] i..ii...iii..iiii.....i...i.........i..iii.............i.....i.....ii...i..i.ii..............i...ii. 100/119
[00:58:18] .ii.i.....iiii.....
[00:58:18] 
[00:58:18]  finished in 3.641
[00:58:18] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:33] 
[00:58:33] running 118 tests
[00:58:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:59:02] ......iii.i.....ii
[00:59:02] 
[00:59:02]  finished in 29.112
[00:59:02] travis_fold:end:test_debuginfo

---
[01:15:16] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:16]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:15:19] error[E0432]: unresolved import `super::ReentrantMutex`
[01:15:19]   --> src/libstd/sys_common/parking_lot/remutex.rs:93:9
[01:15:19]    |
[01:15:19] 93 |     use super::ReentrantMutex;
[01:15:19]    |         ^^^^^^^^^^^^^^^^^^^^^ no `ReentrantMutex` in `sys_common::parking_lot::remutex`. Did you mean to use `RawReentrantMutex`?
[01:15:19] 
[01:15:19] error[E0432]: unresolved imports `Condvar`, `Mutex`
[01:15:19]    --> src/libstd/sys_common/parking_lot/condvar.rs:402:10
[01:15:19]     |
[01:15:19] 402 |     use {Condvar, Mutex};
[01:15:19]     |          ^^^^^^^  ^^^^^ no `Mutex` in the root
[01:15:19]     |          no `Condvar` in the root
[01:15:19] 
[01:15:19] 
j/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
