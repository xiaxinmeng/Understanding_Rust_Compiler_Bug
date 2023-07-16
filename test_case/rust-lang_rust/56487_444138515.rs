plain
travis_time:end:06d8bcdd:start=1543936815307815953,finish=1543936816856564926,duration=1548748973
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:12] 
######################################################################## 100.0%
[00:01:12] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:13]     Updating crates.io index
[00:01:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:19] Build completed unsuccessfully in 0:00:18
[00:01:19] make: *** [prepare] Error 1
[00:01:19] Makefile:81: recipe for target 'prepare' failed
[00:01:20] Command failed. Attempt 2/5:
[00:01:20] Command failed. Attempt 2/5:
[00:01:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:21] Build completed unsuccessfully in 0:00:00
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:21] make: *** [prepare] Error 1
[00:01:23] Command failed. Attempt 3/5:
[00:01:23] Command failed. Attempt 3/5:
[00:01:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:23] Build completed unsuccessfully in 0:00:00
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] make: *** [prepare] Error 1
[00:01:26] Command failed. Attempt 4/5:
[00:01:26] Command failed. Attempt 4/5:
[00:01:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] make: *** [prepare] Error 1
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:30] Command failed. Attempt 5/5:
[00:01:30] Command failed. Attempt 5/5:
[00:01:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:30] The command has failed after 5 attempts.
---
travis_time:end:0c63b4b0:start=1543936917912404299,finish=1543936917916915405,duration=4511106
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03f99cfc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c88f47e
travis_time:start:0c88f47e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022e3788
$ dmesg | grep -i kill
