plain
travis_time:end:240831fc:start=1544628339672413278,finish=1544628340791953524,duration=1119540246
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
###########################################                               61.0%
######################################################################## 100.0%
[00:01:11] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:11]     Updating crates.io index
[00:01:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:19] Build completed unsuccessfully in 0:00:36
[00:01:19] Makefile:81: recipe for target 'prepare' failed
[00:01:19] make: *** [prepare] Error 1
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
[00:01:23] make: *** [prepare] Error 1
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:26] Command failed. Attempt 4/5:
[00:01:26] Command failed. Attempt 4/5:
[00:01:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:26] Build completed unsuccessfully in 0:00:00
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] make: *** [prepare] Error 1
[00:01:30] Command failed. Attempt 5/5:
[00:01:30] Command failed. Attempt 5/5:
[00:01:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:81: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:31] The command has failed after 5 attempts.
---
travis_time:end:117b4ba9:start=1544628441684765542,finish=1544628441693001434,duration=8235892
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0401e2eb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06fb4ed6
travis_time:start:06fb4ed6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1403779a
$ dmesg | grep -i kill
