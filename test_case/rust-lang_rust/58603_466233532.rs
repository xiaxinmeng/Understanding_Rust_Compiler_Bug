plain
travis_time:end:226d6c44:start=1550794683997335882,finish=1550794792576769883,duration=108579434001
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:52:28] travis_time:end:stage2-rustdoc:start=1550797786203273280,finish=1550797950072796222,duration=163869522942

[00:52:28] Build completed successfully in 0:48:18
[00:52:28]     Finished dev [unoptimized] target(s) in 0.35s
[00:52:30] thread 'main' panicked at 'file not found: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:997:5
[00:52:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:52:30] Build completed unsuccessfully in 0:00:01
[00:52:30] Makefile:18: recipe for target 'all' failed
[00:52:30] Makefile:18: recipe for target 'all' failed
[00:52:30] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d5a7ca6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 01:12:32 UTC 2019
