plain
travis_time:end:05ce9fc6:start=1558890473997572039,finish=1558890474789162005,duration=791589966
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:56] 
######################################################################## 100.0%
[00:01:56] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:56]     Updating crates.io index
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:31
[00:02:14] Makefile:69: recipe for target 'prepare' failed
[00:02:14] make: *** [prepare] Error 1
[00:02:15] Command failed. Attempt 2/5:
[00:02:15] Command failed. Attempt 2/5:
[00:02:16]     Updating crates.io index
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:01
[00:02:17] Makefile:69: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:19] Command failed. Attempt 3/5:
[00:02:19] Command failed. Attempt 3/5:
[00:02:19]     Updating crates.io index
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
[00:02:23] Command failed. Attempt 4/5:
[00:02:23] Command failed. Attempt 4/5:
[00:02:23]     Updating crates.io index
[00:02:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:24] Build completed unsuccessfully in 0:00:01
[00:02:24] make: *** [prepare] Error 1
[00:02:24] Makefile:69: recipe for target 'prepare' failed
[00:02:28] Command failed. Attempt 5/5:
[00:02:28] Command failed. Attempt 5/5:
[00:02:28]     Updating crates.io index
[00:02:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] make: *** [prepare] Error 1
[00:02:29] Makefile:69: recipe for target 'prepare' failed
[00:02:29] The command has failed after 5 attempts.
---
travis_time:end:192524cf:start=1558890635694314237,finish=1558890635699444172,duration=5129935
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18a3a340
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10a51422
travis_time:start:10a51422
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:004a89dd
$ dmesg | grep -i kill
