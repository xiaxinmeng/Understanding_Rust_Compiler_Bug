plain
travis_time:end:168c05d0:start=1545258900620362070,finish=1545258902133977977,duration=1513615907
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
[00:58:18] 
[00:58:18] running 119 tests
[00:58:41] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:58:45] i......iii.i.....ii
[00:58:45] 
[00:58:45]  finished in 27.123
[00:58:45] travis_fold:end:test_debuginfo

---
[01:10:01] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:01]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:10:08] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:10:08]      |
[01:10:08]      |
[01:10:08] 1023 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[v.len() - 1 - i]);
[01:10:08]      |
[01:10:08]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:10:08] 
[01:10:08] 
[01:10:08] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:10:08]      |
[01:10:08]      |
[01:10:08] 1025 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:10:08]      |
[01:10:08]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:10:08] 
[01:10:08] 
[01:10:08] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:10:08]      |
[01:10:08]      |
[01:10:08] 1032 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[i]);
[01:10:08]      |
[01:10:08]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:10:08] 
[01:10:08] 
[01:10:08] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:10:08]      |
[01:10:08]      |
[01:10:08] 1034 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:10:08]      |
[01:10:08]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:10:08] 
Wed, 19 Dec 2018 23:45:26 GMT
---
travis_time:end:001bc4e3:start=1545263128121983093,finish=1545263128129852949,duration=7869856
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02e725ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EX
