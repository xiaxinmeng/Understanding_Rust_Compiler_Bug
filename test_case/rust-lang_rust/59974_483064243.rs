plain
travis_time:end:00155b28:start=1555277477585530997,finish=1555277478355469149,duration=769938152
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:32]    Compiling serde_derive v1.0.81
[01:03:53]    Compiling failure v0.1.5
[01:03:55]    Compiling rustfix v0.4.4
[01:03:56]    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
[01:03:56] error[E0063]: missing field `exclude_should_panic` in initializer of `test::TestOpts`
[01:03:56]     |
[01:03:56] 524 |     test::TestOpts {
[01:03:56]     |     ^^^^^^^^^^^^^^ missing `exclude_should_panic`
[01:03:56] 
---
[01:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--message-format" "json"
[01:03:57] expected success, got: exit code: 101
[01:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:57] Build completed unsuccessfully in 0:00:47
[01:03:57] Makefile:48: recipe for target 'check' failed
[01:03:57] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3274aa58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 14 22:35:26 UTC 2019
---
travis_time:end:06e7ffdf:start=1555281327987008795,finish=1555281327993191075,duration=6182280
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1fd7e6ec
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:067195aa
travis_time:start:067195aa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08e7e260
$ dmesg | grep -i kill
