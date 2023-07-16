plain
travis_time:end:0b11c1e5:start=1545258922994601671,finish=1545258924038627201,duration=1044025530
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:36] 
[00:52:36] running 119 tests
[00:52:57] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:53:01] i......iii.i.....ii
[00:53:01] 
[00:53:01]  finished in 25.633
[00:53:01] travis_fold:end:test_debuginfo

---
[01:03:21] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:22]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:03:29] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:03:29]      |
[01:03:29]      |
[01:03:29] 1023 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[v.len() - 1 - i]);
[01:03:29]      |
[01:03:29]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:03:29] 
[01:03:29] 
[01:03:29] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:03:29]      |
[01:03:29]      |
[01:03:29] 1025 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:03:29]      |
[01:03:29]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:03:29] 
[01:03:29] 
[01:03:29] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:03:29]      |
[01:03:29]      |
[01:03:29] 1032 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[i]);
[01:03:29]      |
[01:03:29]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:03:29] 
[01:03:29] 
[01:03:29] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:03:29]      |
[01:03:29]      |
[01:03:29] 1034 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:03:29]      |
[01:03:29]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:03:29] 
---
travis_time:end:083043ed:start=1545262752911802907,finish=1545262752919849697,duration=8046790
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05ed8440
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE"travis_time:end:05ed8440:start=1545262752928073354,finish=1545262753026913455,duration=98840101
travis_fold:start:after_failure.5
travis_time:start:02c2f3fe
travis_time:start:02c2f3fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08956c4a
$ dmesg | grep -i kill
