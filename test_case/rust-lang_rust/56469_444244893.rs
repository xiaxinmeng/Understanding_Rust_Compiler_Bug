plain
travis_time:end:0b3bda27:start=1543950677488543630,finish=1543950732835752600,duration=55347208970
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
[00:55:08] 
[00:55:08] running 120 tests
[00:55:11] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:11] ..ii.i.....iiii.....
[00:55:11] 
[00:55:11]  finished in 3.417
[00:55:11] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:26] 
[00:55:26] running 118 tests
[00:55:51] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:55] ......iii.i.....ii
[00:55:55] 
[00:55:55]  finished in 29.721
[00:55:55] travis_fold:end:test_debuginfo

---
[01:11:41]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:11:45] error: unused import: `Cell`
[01:11:45]   --> src/libstd/panicking.rs:25:12
[01:11:45]    |
[01:11:45] 25 | use cell::{Cell, RefCell};
[01:11:45]    |
[01:11:45]    = note: `-D unused-imports` implied by `-D warnings`
[01:11:45] 
travis_time:end:02e8a810:start=1543950741895192427,finish=1543955059019440207,duration=4317124247780
---
travis_time:end:034f1f70:start=1543955061691907135,finish=1543955061793721149,duration=101814014
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08a2ba78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:30641268
$ dmesg | grep -i kill
