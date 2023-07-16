plain
travis_time:end:0147c006:start=1545254357811000030,finish=1545254358852659705,duration=1041659675
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
[00:55:15] 
[00:55:15] running 119 tests
[00:55:38] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:55:42] i......iii.i.....ii
[00:55:42] 
[00:55:42]  finished in 27.317
[00:55:42] travis_fold:end:test_debuginfo

---
[01:06:20] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:21]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:06:28] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:06:28]      |
[01:06:28]      |
[01:06:28] 1023 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[v.len() - 1 - i]);
[01:06:28]      |
[01:06:28]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:06:28] 
[01:06:28] 
[01:06:28] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:06:28]      |
[01:06:28]      |
[01:06:28] 1025 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:06:28]      |
[01:06:28]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:06:28] 
[01:06:28] 
[01:06:28] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:06:28]      |
[01:06:28]      |
[01:06:28] 1032 |         assert_eq!(v.iter().nth_back(i).unwrap(), &v[i]);
[01:06:28]      |
[01:06:28]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:06:28] 
[01:06:28] 
[01:06:28] error[E0658]: use of unstable library feature 'iter_nth_back' (see issue #56995)
[01:06:28]      |
[01:06:28]      |
[01:06:28] 1034 |     assert_eq!(v.iter().nth_back(v.len()), None);
[01:06:28]      |
[01:06:28]      = help: add #![feature(iter_nth_back)] to the crate attributes to enable
[01:06:28] 
[01:06:36] error: aborting due to 4 previous errors
[01:06:36] error: aborting due to 4 previous errors
[01:06:36] 
[01:06:36] For more information about this error, try `rustc --explain E0658`.
[01:06:36] error: Could not compile `core`.
[01:06:36] 
[01:06:36] To learn more, run the command again with --verbose.
[01:06:36] 
[01:06:36] 
[01:06:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:06:36] 
[01:06:36] 
[01:06:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:36] Build completed unsuccessfully in 0:22:29
[01:06:36] Build completed unsuccessfully in 0:22:29
[01:06:36] Makefile:58: recipe for target 'check' failed
[01:06:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25bcb77a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 22:26:04 UTC 2018
---
travis_time:end:135543d1:start=1545258365786445655,finish=1545258365795029893,duration=8584238
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ffd4fa3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EX
