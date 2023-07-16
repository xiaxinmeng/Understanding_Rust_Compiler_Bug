plain
travis_time:end:0878cf58:start=1549915340247387183,finish=1549915348378965833,duration=8131578650
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
tidy check
[00:04:15] * 565 error codes
[00:04:15] * highest error code: E0722
[00:04:16] * 250 features
[00:04:16] crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
[00:04:16]   * rustc-ap-syntax 306.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:16]   * rustc-ap-syntax 366.0.0 (registry+https://github.com/rust-lang/crates.io-index)
[00:04:16] some tidy checks failed
[00:04:16] 
[00:04:16] 
[00:04:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:16] 
[00:04:16] 
[00:04:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:16] Build completed unsuccessfully in 0:00:46
[00:04:16] Build completed unsuccessfully in 0:00:46
[00:04:16] make: *** [tidy] Error 1
[00:04:16] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:179edd0f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 20:07:01 UTC 2019
---
travis_time:end:1287e9b0:start=1549915622149608599,finish=1549915622154391600,duration=4783001
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ef71388
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c14a7a1
travis_time:start:0c14a7a1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1153c736
$ dmesg | grep -i kill
