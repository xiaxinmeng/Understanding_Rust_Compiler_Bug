plain
travis_time:end:00bfe1ef:start=1545472031155931615,finish=1545472088481215912,duration=57325284297
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:13] 
######################################################################## 100.0%
[00:01:13] extracting /checkout/obj/build/cache/2018-12-09/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:13]     Updating crates.io index
[00:01:21]     Updating git repository `https://github.com/servo/rust-smallvec`
[00:01:21] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:21] Build completed unsuccessfully in 0:00:19
[00:01:21] Makefile:81: recipe for target 'prepare' failed
[00:01:21] make: *** [prepare] Error 1
[00:01:22] Command failed. Attempt 2/5:
[00:01:22] Command failed. Attempt 2/5:
[00:01:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:23] Build completed unsuccessfully in 0:00:00
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] make: *** [prepare] Error 1
[00:01:25] Command failed. Attempt 3/5:
[00:01:25] Command failed. Attempt 3/5:
[00:01:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:25] Build completed unsuccessfully in 0:00:00
[00:01:25] Makefile:81: recipe for target 'prepare' failed
[00:01:25] make: *** [prepare] Error 1
[00:01:28] Command failed. Attempt 4/5:
[00:01:28] Command failed. Attempt 4/5:
[00:01:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:28] make: *** [prepare] Error 1
[00:01:32] Command failed. Attempt 5/5:
[00:01:32] Command failed. Attempt 5/5:
[00:01:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:33] Build completed unsuccessfully in 0:00:00
[00:01:33] Makefile:81: recipe for target 'prepare' failed
[00:01:33] make: *** [prepare] Error 1
[00:01:33] The command has failed after 5 attempts.
---
travis_time:end:1358601c:start=1545472190508279787,finish=1545472190514602852,duration=6323065
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a4bf021
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025c5ae8
travis_time:start:025c5ae8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:181c644c
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.762600] init: failsafe main process (1094) killed by TERM signal
[   42.170072] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
