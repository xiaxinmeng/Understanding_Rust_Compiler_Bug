plain
travis_time:end:272558b7:start=1542116151346096217,finish=1542116152369822563,duration=1023726346
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---

[00:32:26] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:32:27] tidy error: /checkout/src/libstd/sys/unix/process/process_unix.rs:24: line longer than 100 chars
[00:32:28] some tidy checks failed
[00:32:28] 
[00:32:28] 
[00:32:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:32:28] 
[00:32:28] 
[00:32:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:32:28] Build completed unsuccessfully in 0:00:51
[00:32:28] Build completed unsuccessfully in 0:00:51
[00:32:28] Makefile:79: recipe for target 'tidy' failed
[00:32:28] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11a0edc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 14:08:30 UTC 2018
---
travis_time:end:34d0c4ae:start=1542118110824780071,finish=1542118110838037945,duration=13257874
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:024c5572
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d001e01
$ dmesg | grep -i kill
