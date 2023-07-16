plain
travis_time:end:13656db4:start=1549570665664060566,finish=1549570667929761829,duration=2265701263
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:24:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:24:08] expected success, got: exit code: 101
[00:24:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:08] Build completed unsuccessfully in 0:19:19
[00:24:08] make: *** [all] Error 1
[00:24:08] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02a0a8d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 20:42:06 UTC 2019
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:080d1b13
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot acng_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b23aa2b
$ dmesg | grep -i kill
