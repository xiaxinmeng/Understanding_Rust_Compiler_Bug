plain
travis_time:end:00e8faaa:start=1548687619942773618,finish=1548687622404067823,duration=2461294205
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:25] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1729: trailing whitespace
[00:05:27] some tidy checks failed
[00:05:27] 
[00:05:27] 
[00:05:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:27] 
[00:05:27] 
[00:05:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:27] Build completed unsuccessfully in 0:00:47
[00:05:27] Build completed unsuccessfully in 0:00:47
[00:05:27] make: *** [tidy] Error 1
[00:05:27] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d245d16
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 15:06:00 UTC 2019
