plain
travis_time:end:2c1eab00:start=1555216779568342092,finish=1555216780482230659,duration=913888567
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####################################################################      95.2%
######################################################################## 100.0%
[00:01:42] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:42]     Updating crates.io index
[00:01:57] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:57] Build completed unsuccessfully in 0:00:40
[00:01:57] make: *** [prepare] Error 1
[00:01:57] Makefile:69: recipe for target 'prepare' failed
[00:01:58] Command failed. Attempt 2/5:
[00:01:58] Command failed. Attempt 2/5:
[00:01:59] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:59] Build completed unsuccessfully in 0:00:00
[00:01:59] make: *** [prepare] Error 1
[00:01:59] Makefile:69: recipe for target 'prepare' failed
[00:02:01] Command failed. Attempt 3/5:
[00:02:01] Command failed. Attempt 3/5:
[00:02:01] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:00
[00:02:01] Makefile:69: recipe for target 'prepare' failed
[00:02:01] make: *** [prepare] Error 1
[00:02:04] Command failed. Attempt 4/5:
[00:02:04] Command failed. Attempt 4/5:
[00:02:05] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:05] Build completed unsuccessfully in 0:00:00
[00:02:05] Makefile:69: recipe for target 'prepare' failed
[00:02:05] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 5/5:
[00:02:09] Command failed. Attempt 5/5:
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] Makefile:69: recipe for target 'prepare' failed
[00:02:09] make: *** [prepare] Error 1
[00:02:09] The command has failed after 5 attempts.
---
travis_time:end:05ebe000:start=1555216923310323757,finish=1555216923316528080,duration=6204323
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005e9bf6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16074ff6
travis_time:start:16074ff6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c05dd74
$ dmesg | grep -i kill
