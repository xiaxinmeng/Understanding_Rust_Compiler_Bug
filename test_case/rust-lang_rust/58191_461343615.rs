plain
travis_time:end:006c3798:start=1549530966916174617,finish=1549530969902762860,duration=2986588243
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:48] expected success, got: exit code: 101
[00:06:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:48] Build completed unsuccessfully in 0:02:32
[00:06:48] make: *** [all] Error 1
[00:06:48] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00d8e130
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 09:23:11 UTC 2019
---
travis_time:end:03c2a77a:start=1549531392444493598,finish=1549531392449966431,duration=5472833
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fdbbc7f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b214c2c
travis_time:start:1b214c2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt
