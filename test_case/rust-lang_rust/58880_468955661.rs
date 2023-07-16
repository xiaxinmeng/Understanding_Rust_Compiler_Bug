plain
travis_time:end:02463780:start=1551557094087658813,finish=1551557095031726060,duration=944067247
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
######################################                                    53.6%
######################################################################## 100.0%
[00:01:55] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:55]     Updating crates.io index
[00:02:06]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:07] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:07] Build completed unsuccessfully in 0:00:30
[00:02:07] make: *** [prepare] Error 1
[00:02:07] Makefile:70: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 2/5:
[00:02:08] Command failed. Attempt 2/5:
[00:02:08]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:11] Command failed. Attempt 3/5:
[00:02:11] Command failed. Attempt 3/5:
[00:02:11]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:11] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:11] Build completed unsuccessfully in 0:00:00
[00:02:11] Makefile:70: recipe for target 'prepare' failed
[00:02:11] make: *** [prepare] Error 1
[00:02:14] Command failed. Attempt 4/5:
[00:02:14] Command failed. Attempt 4/5:
[00:02:14]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:15] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:00
[00:02:15] make: *** [prepare] Error 1
[00:02:15] Makefile:70: recipe for target 'prepare' failed
[00:02:19] Command failed. Attempt 5/5:
[00:02:19] Command failed. Attempt 5/5:
[00:02:19]     Updating git repository `https://github.com/mati865/compiletest-rs.git`
[00:02:19] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] make: *** [prepare] Error 1
[00:02:19] Makefile:70: recipe for target 'prepare' failed
[00:02:19] The command has failed after 5 attempts.
---
travis_time:end:155f8188:start=1551557249011084735,finish=1551557249016350819,duration=5266084
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05ead28e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01f677d6
travis_time:start:01f677d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f3a7ae0
$ dmesg | grep -i kill
