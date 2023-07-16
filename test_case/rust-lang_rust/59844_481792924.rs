plain
travis_time:end:0225c4a8:start=1554914663959517361,finish=1554914664852855989,duration=893338628
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
make -j 4 check
[01:02:45]     Finished dev [unoptimized] target(s) in 0.36s
[01:02:46] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:02:46]   left: `2`,
[01:02:46]  right: `1`: Only ever build one copy of rustdoc per run, currently built in stages [2, 1]', src/bootstrap/tool.rs:464:17
[01:02:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:46] Build completed unsuccessfully in 0:00:02
[01:02:46] Build completed unsuccessfully in 0:00:02
[01:02:47] make: *** [check] Error 1
[01:02:47] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00bd1ccf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 17:47:24 UTC 2019
