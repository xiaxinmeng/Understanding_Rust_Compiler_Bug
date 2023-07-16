plain
travis_time:end:1c2591e8:start=1553617988089025993,finish=1553617989429422018,duration=1340396025
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:07] 
######################################################################## 100.0%
[00:02:07] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:07]     Updating crates.io index
[00:02:21]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:23]   Downloaded petgraph v0.4.13
[00:02:23]   Downloaded toml v0.4.10
[00:02:23]   Downloaded time v0.1.40
[00:02:23]   Downloaded cc v1.0.28
---
tidy check
[00:04:16] * 569 error codes
[00:04:16] * highest error code: E0725
[00:04:16] * 252 features
[00:04:17] invalid source: "git+https://github.com/gnzlbg/libtest?branch=clippy_ci#719256356b565a138e0005cabbc56153e5902dca"
[00:04:17] some tidy checks failed
[00:04:17] 
[00:04:17] 
[00:04:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:17] 
[00:04:17] 
[00:04:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:17] Build completed unsuccessfully in 0:00:47
[00:04:17] Build completed unsuccessfully in 0:00:47
[00:04:17] make: *** [tidy] Error 1
[00:04:17] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:072b6e3c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 16:37:37 UTC 2019
---
travis_time:end:04d6f76e:start=1553618258837335839,finish=1553618258842729901,duration=5394062
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f832848
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0be836ec
travis_time:start:0be836ec
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03b5da4e
$ dmesg | grep -i kill
