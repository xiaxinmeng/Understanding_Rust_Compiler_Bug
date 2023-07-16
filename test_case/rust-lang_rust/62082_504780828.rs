plain
travis_time:end:02f67a5c:start=1561318618410054327,finish=1561318620673307163,duration=2263252836
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
################################################                          66.8%
######################################################################## 100.0%
[00:02:02] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:02]     Updating crates.io index
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:35
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:69: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 2/5:
[00:02:22] Command failed. Attempt 2/5:
[00:02:22]     Updating crates.io index
[00:02:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:00
[00:02:23] make: *** [prepare] Error 1
[00:02:23] Makefile:69: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 3/5:
[00:02:25] Command failed. Attempt 3/5:
[00:02:25]     Updating crates.io index
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:69: recipe for target 'prepare' failed
[00:02:28] Command failed. Attempt 4/5:
[00:02:28] Command failed. Attempt 4/5:
[00:02:28]     Updating crates.io index
[00:02:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] Makefile:69: recipe for target 'prepare' failed
[00:02:29] make: *** [prepare] Error 1
[00:02:33] Command failed. Attempt 5/5:
[00:02:33] Command failed. Attempt 5/5:
[00:02:33]     Updating crates.io index
[00:02:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:69: recipe for target 'prepare' failed
[00:02:33] The command has failed after 5 attempts.
---
travis_time:end:113e305f:start=1561318786598508711,finish=1561318786605512513,duration=7003802
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2873d831
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:109f8597
travis_time:start:109f8597
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:058329dd
$ dmesg | grep -i kill
