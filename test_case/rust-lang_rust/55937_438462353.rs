plain
travis_time:end:0c31544c:start=1542146451797236621,finish=1542146511332127771,duration=59534891150
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:21:41]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:23:07]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:23:16]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
0:43
[00:25:00] make: *** [all] Error 1
[00:25:00] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e5025c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 22:27:01 UTC 2018
