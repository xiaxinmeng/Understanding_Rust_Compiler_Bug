plain
travis_time:end:02f3ffb8:start=1547742535837093475,finish=1547742537073951128,duration=1236857653
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:30] expected success, got: exit code: 101
[00:03:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:30] Build completed unsuccessfully in 0:00:06
[00:03:30] make: *** [all] Error 1
[00:03:30] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07e3fb53
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 16:32:39 UTC 2019
Thu Jan 17 16:32:39 UTC 2019
|| true
travis_time:end:01d67b88:start=1547742759827842676,finish=1547742759834120533,duration=6277857
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3e320ce4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c2e68b9
$ dmesg | grep -i kill
