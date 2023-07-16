plain
travis_time:end:10e79dee:start=1553595038601530298,finish=1553595039645057479,duration=1043527181
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#############################                                             40.9%
######################################################################## 100.0%
[00:01:41] extracting /checkout/obj/build/cache/2019-02-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:41]     Updating crates.io index
[00:02:01] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:38
[00:02:01] Makefile:69: recipe for target 'prepare' failed
[00:02:01] make: *** [prepare] Error 1
[00:02:02] Command failed. Attempt 2/5:
[00:02:02] Command failed. Attempt 2/5:
[00:02:02]     Updating crates.io index
[00:02:03] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:03] Build completed unsuccessfully in 0:00:01
[00:02:03] Makefile:69: recipe for target 'prepare' failed
[00:02:03] make: *** [prepare] Error 1
[00:02:05] Command failed. Attempt 3/5:
[00:02:05] Command failed. Attempt 3/5:
[00:02:05]     Updating crates.io index
[00:02:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] make: *** [prepare] Error 1
[00:02:06] Makefile:69: recipe for target 'prepare' failed
[00:02:09] Command failed. Attempt 4/5:
[00:02:09] Command failed. Attempt 4/5:
[00:02:09]     Updating crates.io index
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:69: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 5/5:
[00:02:14] Command failed. Attempt 5/5:
[00:02:14]     Updating crates.io index
[00:02:16] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:02
[00:02:16] Makefile:69: recipe for target 'prepare' failed
[00:02:16] make: *** [prepare] Error 1
[00:02:16] The command has failed after 5 attempts.
---
travis_time:end:0d282924:start=1553595188329610328,finish=1553595188336087389,duration=6477061
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31e27f2c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18258835
travis_time:start:18258835
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e8427f0
$ dmesg | grep -i kill
