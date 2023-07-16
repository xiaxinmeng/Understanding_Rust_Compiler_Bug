plain
travis_time:end:177db0ea:start=1550376680919340253,finish=1550376683142691771,duration=2223351518
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:08] 
######################################################################## 100.0%
[00:02:08] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:08]     Updating crates.io index
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:28
[00:02:19] make: *** [prepare] Error 1
[00:02:19] Makefile:70: recipe for target 'prepare' failed
[00:02:20] Command failed. Attempt 2/5:
[00:02:20] Command failed. Attempt 2/5:
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:00
[00:02:20] Makefile:70: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
[00:02:22] Command failed. Attempt 3/5:
[00:02:22] Command failed. Attempt 3/5:
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:70: recipe for target 'prepare' failed
[00:02:25] Command failed. Attempt 4/5:
[00:02:25] Command failed. Attempt 4/5:
[00:02:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] Makefile:70: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:30] Command failed. Attempt 5/5:
[00:02:30] Command failed. Attempt 5/5:
[00:02:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:30] Build completed unsuccessfully in 0:00:00
[00:02:30] make: *** [prepare] Error 1
[00:02:30] Makefile:70: recipe for target 'prepare' failed
[00:02:30] The command has failed after 5 attempts.
---
travis_time:end:01369c95:start=1550376846919242545,finish=1550376846925535269,duration=6292724
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1056224a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03307f8a
travis_time:start:03307f8a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0028faa8
$ dmesg | grep -i kill
