plain
travis_time:end:2b01e8fc:start=1559299511752065945,finish=1559299512687414178,duration=935348233
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:03:27]   Downloaded curl-sys v0.4.18
[00:03:27]   Downloaded memchr v2.2.0
[00:03:27]   Downloaded rand_os v0.1.3
[00:03:27]   Downloaded glob v0.3.0
[00:03:27]   Downloaded rgb v0.8.13
[00:03:27]   Downloaded num-traits v0.1.43
[00:03:27]   Downloaded env_logger v0.6.1
[00:03:27]   Downloaded error-chain v0.12.1
[00:03:27]   Downloaded datafrog v2.0.1
---
[00:04:47] * rdrand 
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:01:17
---
travis_time:end:01b570f6:start=1559299811869586621,finish=1559299811874630983,duration=5044362
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b263885
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:027ab59d
travis_time:start:027ab59d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06a0ae11
$ dmesg | grep -i kill
