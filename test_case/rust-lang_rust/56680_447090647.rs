plain
travis_time:end:03f7ec03:start=1544728791539899780,finish=1544728800273371335,duration=8733471555
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
################################################################          89.3%
######################################################################## 100.0%
[00:01:28] extracting /checkout/obj/build/cache/2018-11-21/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:29]     Updating crates.io index
[00:01:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:36] Build completed unsuccessfully in 0:00:19
[00:01:36] make: *** [prepare] Error 1
[00:01:36] Makefile:81: recipe for target 'prepare' failed
[00:01:37] Command failed. Attempt 2/5:
[00:01:37] Command failed. Attempt 2/5:
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:00
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:81: recipe for target 'prepare' failed
[00:01:39] Command failed. Attempt 3/5:
[00:01:39] Command failed. Attempt 3/5:
[00:01:40] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:40] Build completed unsuccessfully in 0:00:00
[00:01:40] make: *** [prepare] Error 1
[00:01:40] Makefile:81: recipe for target 'prepare' failed
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] Command failed. Attempt 4/5:
[00:01:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:43] Build completed unsuccessfully in 0:00:00
[00:01:43] make: *** [prepare] Error 1
[00:01:43] Makefile:81: recipe for target 'prepare' failed
[00:01:47] Command failed. Attempt 5/5:
[00:01:47] Command failed. Attempt 5/5:
[00:01:47] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:47] Build completed unsuccessfully in 0:00:00
[00:01:47] make: *** [prepare] Error 1
[00:01:47] Makefile:81: recipe for target 'prepare' failed
[00:01:47] The command has failed after 5 attempts.
---
travis_time:end:00008d78:start=1544728918893631519,finish=1544728918900438953,duration=6807434
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b8a219b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:037cb6d8
travis_time:start:037cb6d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2528b386
$ dmesg | grep -i kill
