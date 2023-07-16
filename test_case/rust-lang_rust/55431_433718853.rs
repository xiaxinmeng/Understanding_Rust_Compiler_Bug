plain
travis_time:end:0f20aec0:start=1540737014979086364,finish=1540737068428273499,duration=53449187135
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:15] .................................................................................................... 2200/4972
[00:51:19] .................................................................................................... 2300/4972
[00:51:23] .................................................................................................... 2400/4972
[00:51:27] .................................................................................................... 2500/4972
[00:51:30] ............................................................iiiiiiiii............................... 2600/4972
[00:51:37] ..........ii........................................................................................ 2800/4972
[00:51:39] .................................................................................................... 2900/4972
[00:51:43] .................................................................................................... 3000/4972
[00:51:46] .....i.............................................................................................. 3100/4972
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:41] 
[01:04:41] running 112 tests
[01:04:44] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:04:44] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:04:44] 
[01:04:44]  finished in 3.451
[01:04:44] travis_fold:end:test_codegen
---
[01:39:47] 
[01:39:47] 
[01:39:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:47] Build completed unsuccessfully in 0:53:20
[01:39:47] make: *** [check] Error 1
[01:39:47] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03b33118
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:133bb880:start=1540743076301033206,finish=1540743076305694415,duration=4661209
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0beb0618
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then pr
