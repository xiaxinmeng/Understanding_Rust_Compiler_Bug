plain
travis_time:end:1bdac210:start=1554458957320418155,finish=1554458958342837446,duration=1022419291
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
tidy check
[00:04:09] * 569 error codes
[00:04:09] * highest error code: E0725
[00:04:09] * 252 features
[00:04:10] invalid source: "git+https://github.com/Zoxc/rustc-hash.git?branch=tweaks#7cdf79648c21699a481b0411b1abf9fcfabb1a3a"
[00:04:10] some tidy checks failed
[00:04:10] 
[00:04:10] 
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] 
[00:04:10] 
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:00:44
[00:04:10] Build completed unsuccessfully in 0:00:44
[00:04:10] make: *** [tidy] Error 1
[00:04:10] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03046314
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr  5 10:13:39 UTC 2019
---
travis_time:end:0f35f124:start=1554459220075989392,finish=1554459220080689328,duration=4699936
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:038983fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0204ba92
travis_time:start:0204ba92
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:125171a8
$ dmesg | grep -i kill
