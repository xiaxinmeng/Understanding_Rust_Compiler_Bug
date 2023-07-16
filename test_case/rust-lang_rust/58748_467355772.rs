plain
travis_time:end:308c9b5d:start=1551171497028421291,finish=1551171497904482057,duration=876060766
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:06] 
######################################################################## 100.0%
[00:05:06] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:05:06]     Updating crates.io index
[00:05:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:19] Build completed unsuccessfully in 0:00:26
[00:05:19] make: *** [prepare] Error 1
[00:05:19] Makefile:70: recipe for target 'prepare' failed
[00:05:20] Command failed. Attempt 2/5:
[00:05:20] Command failed. Attempt 2/5:
[00:05:20]     Updating crates.io index
[00:05:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:20] Build completed unsuccessfully in 0:00:00
[00:05:20] Makefile:70: recipe for target 'prepare' failed
[00:05:20] make: *** [prepare] Error 1
[00:05:22] Command failed. Attempt 3/5:
[00:05:22] Command failed. Attempt 3/5:
[00:05:22]     Updating crates.io index
[00:05:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:23] Build completed unsuccessfully in 0:00:00
[00:05:23] make: *** [prepare] Error 1
[00:05:23] Makefile:70: recipe for target 'prepare' failed
[00:05:26] Command failed. Attempt 4/5:
[00:05:26] Command failed. Attempt 4/5:
[00:05:26]     Updating crates.io index
[00:05:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:26] Build completed unsuccessfully in 0:00:00
[00:05:26] make: *** [prepare] Error 1
[00:05:26] Makefile:70: recipe for target 'prepare' failed
[00:05:30] Command failed. Attempt 5/5:
[00:05:30] Command failed. Attempt 5/5:
[00:05:31]     Updating crates.io index
[00:05:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:05:31] Build completed unsuccessfully in 0:00:00
[00:05:31] make: *** [prepare] Error 1
[00:05:31] Makefile:70: recipe for target 'prepare' failed
[00:05:31] The command has failed after 5 attempts.
---
travis_time:end:0075cc40:start=1551171843181282917,finish=1551171843186072022,duration=4789105
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03dd65aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23717e51
travis_time:start:23717e51
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05288580
$ dmesg | grep -i kill
