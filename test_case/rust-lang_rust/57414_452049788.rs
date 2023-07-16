plain
travis_time:end:0a476ea6:start=1546888651742117400,finish=1546888652689678853,duration=947561453
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
########################################################                  78.9%
######################################################################## 100.0%
[00:02:01] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:01]     Updating crates.io index
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:29
[00:02:10] Makefile:71: recipe for target 'prepare' failed
[00:02:10] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:71: recipe for target 'prepare' failed
[00:02:13] Command failed. Attempt 3/5:
[00:02:13] Command failed. Attempt 3/5:
[00:02:14] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:14] Build completed unsuccessfully in 0:00:00
[00:02:14] Makefile:71: recipe for target 'prepare' failed
[00:02:14] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 4/5:
[00:02:17] Command failed. Attempt 4/5:
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:71: recipe for target 'prepare' failed
[00:02:21] Command failed. Attempt 5/5:
[00:02:21] Command failed. Attempt 5/5:
[00:02:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:21] Build completed unsuccessfully in 0:00:00
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:71: recipe for target 'prepare' failed
[00:02:21] The command has failed after 5 attempts.
---
travis_time:end:3409ac25:start=1546888807456094453,finish=1546888807460675662,duration=4581209
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:022c84c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27167c73
travis_time:start:27167c73
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:007a06ac
$ dmesg | grep -i kill
