plain
travis_time:end:0413038e:start=1548601462533702093,finish=1548601463580519019,duration=1046816926
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
######################################################################    98.6%
######################################################################## 100.0%
[00:02:20] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:20]     Updating crates.io index
[00:02:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:31] Build completed unsuccessfully in 0:00:26
[00:02:31] Makefile:70: recipe for target 'prepare' failed
[00:02:31] make: *** [prepare] Error 1
[00:02:32] Command failed. Attempt 2/5:
[00:02:32] Command failed. Attempt 2/5:
[00:02:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:32] Build completed unsuccessfully in 0:00:00
[00:02:32] Makefile:70: recipe for target 'prepare' failed
[00:02:32] make: *** [prepare] Error 1
[00:02:34] Command failed. Attempt 3/5:
[00:02:34] Command failed. Attempt 3/5:
[00:02:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] make: *** [prepare] Error 1
[00:02:35] Makefile:70: recipe for target 'prepare' failed
[00:02:38] Command failed. Attempt 4/5:
[00:02:38] Command failed. Attempt 4/5:
[00:02:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:38] Build completed unsuccessfully in 0:00:00
[00:02:38] Makefile:70: recipe for target 'prepare' failed
[00:02:38] make: *** [prepare] Error 1
[00:02:42] Command failed. Attempt 5/5:
[00:02:42] Command failed. Attempt 5/5:
[00:02:43] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:43] Build completed unsuccessfully in 0:00:00
[00:02:43] make: *** [prepare] Error 1
[00:02:43] Makefile:70: recipe for target 'prepare' failed
[00:02:43] The command has failed after 5 attempts.
---
travis_time:end:0407f010:start=1548601641032659799,finish=1548601641038532664,duration=5872865
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26c47241
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009ac55a
travis_time:start:009ac55a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a00ce97
$ dmesg | grep -i kill
