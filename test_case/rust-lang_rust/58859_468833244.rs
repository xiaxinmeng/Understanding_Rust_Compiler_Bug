plain
travis_time:end:00c21cf0:start=1551478734211114491,finish=1551478735194276372,duration=983161881
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---

[00:07:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:07:44] tidy error: /checkout/src/libstd/error.rs:268: line longer than 100 chars
[00:07:46] 
[00:07:46] 
[00:07:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:07:46] 
[00:07:46] 
[00:07:46] some tidy checks failed
[00:07:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:07:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:07:46] Build completed unsuccessfully in 0:00:46
[00:07:46] make: *** [tidy] Error 1
[00:07:46] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:220537c4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar  1 22:26:51 UTC 2019
