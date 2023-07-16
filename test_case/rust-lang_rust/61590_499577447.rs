plain
travis_time:end:278bb030:start=1559839369089445410,finish=1559839369906830173,duration=817384763
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
[00:06:06] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:06:06]     Updating crates.io index
[00:06:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:06:26] Build completed unsuccessfully in 0:00:37
[00:06:26] make: *** [prepare] Error 1
[00:06:26] Makefile:69: recipe for target 'prepare' failed
[00:06:27] Command failed. Attempt 2/5:
[00:06:27] Command failed. Attempt 2/5:
[00:06:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:06:27] Build completed unsuccessfully in 0:00:00
[00:06:27] Makefile:69: recipe for target 'prepare' failed
[00:06:27] make: *** [prepare] Error 1
[00:06:29] Command failed. Attempt 3/5:
[00:06:29] Command failed. Attempt 3/5:
[00:06:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:06:29] Build completed unsuccessfully in 0:00:00
[00:06:29] make: *** [prepare] Error 1
[00:06:29] Makefile:69: recipe for target 'prepare' failed
[00:06:32] Command failed. Attempt 4/5:
[00:06:32] Command failed. Attempt 4/5:
[00:06:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:06:32] Build completed unsuccessfully in 0:00:00
[00:06:32] Makefile:69: recipe for target 'prepare' failed
[00:06:32] make: *** [prepare] Error 1
[00:06:36] Command failed. Attempt 5/5:
[00:06:36] Command failed. Attempt 5/5:
[00:06:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:06:36] Build completed unsuccessfully in 0:00:00
[00:06:36] make: *** [prepare] Error 1
[00:06:36] Makefile:69: recipe for target 'prepare' failed
[00:06:36] The command has failed after 5 attempts.
---
travis_time:end:02413ffa:start=1559839778182776331,finish=1559839778188257376,duration=5481045
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11ebc894
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d060dae
travis_time:start:1d060dae
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:31c855d4
$ dmesg | grep -i kill
