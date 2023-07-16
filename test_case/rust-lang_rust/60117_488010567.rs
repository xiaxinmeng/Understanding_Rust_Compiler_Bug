plain
travis_time:end:09bac711:start=1556639763702366685,finish=1556639766828208007,duration=3125841322
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
############################################################              84.1%
######################################################################## 100.0%
[00:02:00] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:00]     Updating crates.io index
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:36
[00:02:17] Makefile:69: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:18] Command failed. Attempt 2/5:
[00:02:18] Command failed. Attempt 2/5:
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] Makefile:69: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:21] Command failed. Attempt 3/5:
[00:02:21] Command failed. Attempt 3/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 4/5:
[00:02:24] Command failed. Attempt 4/5:
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:00
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:69: recipe for target 'prepare' failed
[00:02:28] Command failed. Attempt 5/5:
[00:02:28] Command failed. Attempt 5/5:
[00:02:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] make: *** [prepare] Error 1
[00:02:29] Makefile:69: recipe for target 'prepare' failed
[00:02:29] The command has failed after 5 attempts.
---
travis_time:end:043bf136:start=1556639928462176119,finish=1556639928467860476,duration=5684357
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06ad8b48
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06225df8
travis_time:start:06225df8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17b09905
$ dmesg | grep -i kill
