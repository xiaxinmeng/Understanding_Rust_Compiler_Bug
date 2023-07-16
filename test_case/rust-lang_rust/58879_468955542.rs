plain
travis_time:end:19055aa0:start=1551557006882825399,finish=1551557007877101231,duration=994275832
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:01] 
######################################################################## 100.0%
[00:02:01] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:01]     Updating crates.io index
[00:02:13] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:13] Build completed unsuccessfully in 0:00:25
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:14] Command failed. Attempt 2/5:
[00:02:14] Command failed. Attempt 2/5:
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:00
[00:02:15] make: *** [prepare] Error 1
[00:02:15] Makefile:70: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 3/5:
[00:02:17] Command failed. Attempt 3/5:
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] Makefile:70: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:20] Command failed. Attempt 4/5:
[00:02:20] Command failed. Attempt 4/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:70: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 5/5:
[00:02:25] Command failed. Attempt 5/5:
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:70: recipe for target 'prepare' failed
[00:02:25] The command has failed after 5 attempts.
---
travis_time:end:22439235:start=1551557167784516814,finish=1551557167791240918,duration=6724104
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18d2dc9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:096dd290
travis_time:start:096dd290
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0681d91a
$ dmesg | grep -i kill
