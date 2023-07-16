plain
travis_time:end:0032569e:start=1561346683152672350,finish=1561346685915157101,duration=2762484751
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####                                                                       6.1%
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating crates.io index
[00:02:21]     Updating git repository `https://github.com/Zoxc/mimallocator.git`
[00:02:22]   Downloaded getopts v0.2.19
[00:02:22]   Downloaded cc v1.0.35
[00:02:22]   Downloaded petgraph v0.4.13
[00:02:22]   Downloaded cmake v0.1.38
---
tidy check
[00:04:55] * 576 error codes
[00:04:55] * highest error code: E0731
[00:04:55] * 261 features
[00:04:56] invalid source: "git+https://github.com/Zoxc/mimallocator.git#8ac805a5ecfeb55f89b309aa31038fc02108f570"
[00:04:56] invalid source: "git+https://github.com/Zoxc/mimallocator.git#8ac805a5ecfeb55f89b309aa31038fc02108f570"
[00:04:56] some tidy checks failed
[00:04:56] 
[00:04:56] 
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:04:56] 
[00:04:56] 
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:56] Build completed unsuccessfully in 0:01:15
---
travis_time:end:05c33ff4:start=1561346993711349177,finish=1561346993716547243,duration=5198066
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b25bf88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c3ee0a2
travis_time:start:0c3ee0a2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e8619f5
$ dmesg | grep -i kill
