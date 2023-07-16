plain
travis_time:end:05dca1d5:start=1556800370232319706,finish=1556800372251075153,duration=2018755447
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#######################################################                   76.7%
######################################################################## 100.0%
[00:01:58] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:58]     Updating crates.io index
[00:02:15]     Updating git repository `https://github.com/gankro/hashbrown`
[00:02:17]   Downloaded cc v1.0.35
[00:02:17]   Downloaded num_cpus v1.8.0
[00:02:18]   Downloaded lazy_static v0.2.11
[00:02:18]   Downloaded cmake v0.1.38
---
tidy check
[00:04:10] * 570 error codes
[00:04:10] * highest error code: E0725
[00:04:11] * 254 features
[00:04:11] invalid source: "git+https://github.com/gankro/hashbrown?branch=singleton#99e5ba92e83a6ad40cbc5c9d79dd18a07592ebf1"
[00:04:11] some tidy checks failed
[00:04:11] 
[00:04:11] 
[00:04:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:11] 
[00:04:11] 
[00:04:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:11] Build completed unsuccessfully in 0:00:44
[00:04:11] Build completed unsuccessfully in 0:00:44
[00:04:11] make: *** [tidy] Error 1
[00:04:11] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:020df69c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 12:37:14 UTC 2019
---
travis_time:end:157a3bf8:start=1556800635685996956,finish=1556800635691102696,duration=5105740
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07a24d72
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:066bcd73
travis_time:start:066bcd73
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2703ee32
$ dmesg | grep -i kill
