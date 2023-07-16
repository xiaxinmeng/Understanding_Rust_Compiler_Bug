plain
travis_time:end:1d2ae632:start=1543592683498975424,finish=1543592684509196900,duration=1010221476
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:38] .................ii................................................................................. 3600/5103
[00:49:40] ...................................i................................................................ 3700/5103
[00:49:41] ...........................................................................................i........ 3800/5103
[00:49:42] .................................................................................................... 3900/5103
[00:49:49] ............F....................................................................................... 4000/5103
[00:49:55] .................................................................................................... 4200/5103
[00:49:59] ...................................................................................i................ 4300/5103
[00:50:04] .................................................................................................... 4400/5103
[00:50:08] .................................................................................................... 4500/5103
---
[00:50:27] 
[00:50:27] 
[00:50:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:27] Build completed unsuccessfully in 0:03:56
[00:50:27] Makefile:58: recipe for target 'check' failed
[00:50:27] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:064530e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 16:35:21 UTC 2018
