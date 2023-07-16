plain
travis_time:end:0739316e:start=1543030883689500703,finish=1543030885806197441,duration=2116696738
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
#####################################################################     96.2%
######################################################################## 100.0%
[00:00:55] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:00:55]     Updating crates.io index
[00:01:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:02] Build completed unsuccessfully in 0:00:20
[00:01:02] make: *** [prepare] Error 1
[00:01:02] Makefile:81: recipe for target 'prepare' failed
[00:01:03] Command failed. Attempt 2/5:
[00:01:03] Command failed. Attempt 2/5:
[00:01:03]     Updating crates.io index
[00:01:03] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:03] Build completed unsuccessfully in 0:00:00
[00:01:03] make: *** [prepare] Error 1
[00:01:03] Makefile:81: recipe for target 'prepare' failed
[00:01:05] Command failed. Attempt 3/5:
[00:01:05] Command failed. Attempt 3/5:
[00:01:06]     Updating crates.io index
[00:01:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:06] Build completed unsuccessfully in 0:00:00
[00:01:06] make: *** [prepare] Error 1
[00:01:06] Makefile:81: recipe for target 'prepare' failed
[00:01:09] Command failed. Attempt 4/5:
[00:01:09] Command failed. Attempt 4/5:
[00:01:09]     Updating crates.io index
[00:01:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:09] Build completed unsuccessfully in 0:00:00
[00:01:09] Makefile:81: recipe for target 'prepare' failed
[00:01:09] make: *** [prepare] Error 1
[00:01:13] Command failed. Attempt 5/5:
[00:01:13] Command failed. Attempt 5/5:
[00:01:14]     Updating crates.io index
[00:01:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:14] Build completed unsuccessfully in 0:00:00
[00:01:14] Makefile:81: recipe for target 'prepare' failed
[00:01:14] make: *** [prepare] Error 1
[00:01:14] The command has failed after 5 attempts.
---
travis_time:end:028c44c6:start=1543030970357399210,finish=1543030970364469771,duration=7070561
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13935b1e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:013f6758
travis_time:start:013f6758
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:174c9fa8
$ dmesg | grep -i kill
